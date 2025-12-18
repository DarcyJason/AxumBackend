use uaparser_rs::UAParser;

pub fn parse_user_agent(user_agent: &str) -> (String, String) {
    let uap = UAParser::from_yaml("./regexes.yaml").unwrap();
    let client = uap.parse(user_agent);
    let device_name = format!("{} on {}", client.user_agent.family, client.os.family);
    let device_type = client.device.family;
    (device_name, device_type)
}
