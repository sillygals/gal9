use fluent::{FluentBundle, FluentResource};
use unic_langid::langid;

fn main() {
    let ftl_string = "welcome = 华人牌2060款手机傻妞为您服务！请输入开机密码：".to_owned();
    let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");
    let langid_cn = langid!("zh-Hans");
    let mut bundle = FluentBundle::new(vec![langid_cn]);
    bundle
        .add_resource(&res)
        .expect("Failed to add FTL resources to the bundle.");
    let msg = bundle
        .get_message("welcome")
        .expect("Message doesn't exist.");
    if let Some(value) = msg.value() {
        let mut err = vec![];
        println!("{}", bundle.format_pattern(value, None, &mut err));
    }
    loop {}
}
