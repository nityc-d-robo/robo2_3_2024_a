use safe_drive::{
    context::Context, error::DynError, logger::Logger, msg::common_interfaces::geometry_msgs::msg,
    msg::common_interfaces::sensor_msgs,
};

#[allow(unused_imports)]
use safe_drive::pr_info;

use differential_two_wheel_control::{Chassis, DtwcSetting, Tire};
use motor_controller::udp_communication;
const CHASSIS: Chassis = Chassis {
    l: Tire { id: 0, raito: 1. },
    r: Tire { id: 1, raito: 1. },
};
const MAX_PAWER_INPUT: f64 = 160.;
const MAX_PAWER_OUTPUT: f64 = 1.;
const MAX_REVOLUTION: f64 = 5400.;

fn main() -> Result<(), DynError> {
    let dtwc_setting = DtwcSetting {
        chassis: CHASSIS,
        max_pawer_input: MAX_PAWER_INPUT,
        max_pawer_output: MAX_PAWER_OUTPUT,
        max_revolution: MAX_REVOLUTION,
    };

    // for debug
    let _logger = Logger::new("robo2_3_2024_a");
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node("robo2_3_2024_a", None, Default::default())?;

    let subscriber_cmd = node.create_subscriber::<msg::Twist>("cmd_vel2_3", None)?;
    let subscriber_joy = node.create_subscriber::<sensor_msgs::msg::Joy>("rjoy2_3", None)?;

    selector.add_subscriber(
        subscriber_cmd,
        Box::new(move |msg| {
            let motor_power = dtwc_setting.move_chassis(msg.linear.x, msg.linear.y, msg.angular.z);

            for i in motor_power.keys() {
                udp_communication::send_pwm_udp("50007", "192.168.1.7:60000", *i, motor_power[i]);
            }
        }),
    );

    // selector.add_subscriber(
    //     subscriber_joy,
    //     Box::new(move |_msg| {
    //         todo!();
    //     }),
    // );

    loop {
        selector.wait()?;
    }
}
