use dbus::{
   blocking::Connection, Error, arg,
};
use std::time::Duration;

pub fn print_dbus_msg(err: Error) {
   eprintln!("======= D-Bus error =======");
   eprintln!("Name: {}", err.name().unwrap());
   eprintln!("Message: {}", err.message().unwrap());
}

pub fn dbus_call(service: &str, path: &str, interface: &str, conn: &Connection, method: &str, dur: Duration) -> Result<bool, Error> {
   let proxy = conn.with_proxy(service, path, dur);
   let (result,) = proxy.method_call(interface, method, ())?;
   Ok(result)
}

pub fn dbus_call_init_system(service: &str, path: &str, interface: &str, conn: &Connection, method: &str, need_arg: Option<bool>, dur: Duration) -> Result<bool, Error> {
   let proxy = conn.with_proxy(service, path, dur);

   let result: Result<(String,), Error> = match need_arg {
      Some(arg) => proxy.method_call(interface, method, (arg,)),
      None => proxy.method_call(interface, method, ())
   };
   match result {
      Ok((res,)) => {
         println!("systemd: {} = {}", method, res);
         Ok(res.as_str() == "yes" || res.as_str() == "challenge")
      },
      Err(err) => Err(err)
   }
}

pub fn dbus_call_init_system_with_args(service: &str, path: &str, interface: &str, conn: &Connection, method: &str, args: Option<&str>, dur: Duration) -> Result<bool, Error> {
   let proxy = conn.with_proxy(service, path, dur);

   let result: Result<(String,), Error> = match args {
      Some(arg) => proxy.method_call(interface, method, (arg,)),
      None => proxy.method_call(interface, method, ())
   };
   match result {
      Ok((res,)) => {
         println!("systemd: {} = {}", method, res);
         Ok(res.as_str() == "yes" || res.as_str() == "challenge")
      },
      Err(err) => Err(err)
   }
}

pub fn dbus_get_property(service: &str, path: &str, interface: &str, conn: &Connection, prop: &str, dur: Duration) -> Result<Box<dyn arg::RefArg>, Error> {
   let proxy = conn.with_proxy(service, path, dur);
   use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
   proxy.get(interface, prop)
}