use anyhow::Result;
use dbus::channel::MatchingReceiver;
use dbus_crossroads::Crossroads;
use dbus_tokio::connection;

const SERVICE_NAME: &str = "org.relm.Identity";
const OBJECT_PATH: &str = "/org/relm/relmidd";
const INTERFACE_NAME: &str = "org.relm.relmidd";

#[tokio::main]
async fn main() -> Result<()> {

    let (resource, conn) = connection::new_system_sync()?;

    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    conn.request_name(SERVICE_NAME, false, true, false).await?;

    println!("DBus service registered as {}", SERVICE_NAME);

    let mut cr = Crossroads::new();

    let iface_token = cr.register(INTERFACE_NAME, |b| {
        b.method("Ping", (), ("response",), |_ctx, _data, ()| {
            Ok(("Pong".to_string(),))
        },);

        b.method("Echo", ("message",), ("response",), |_ctx, _data, (msg,): (String,)| {
            Ok((format!("Echo: {}", msg),))
        },);
    });

    cr.insert(OBJECT_PATH, &[iface_token], ());

    conn.start_receive(
        dbus::message::MatchRule::new_method_call(),
        Box::new(move |msg, conn| {
            cr.handle_message(msg, conn).unwrap();
            true
        }),
    );

    println!("Listening on {}", OBJECT_PATH);

    futures::future::pending::<()>().await;

    #[allow(unreachable_code)]
    Ok(())
}