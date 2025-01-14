  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct JointCommand {

                              pub position: f64,

                          }

                          impl WrappedTypesupport for JointCommand { 

            type CStruct = pendulum_msgs__msg__JointCommand; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pendulum_msgs__msg__JointCommand() }
            }

            fn create_msg() -> *mut pendulum_msgs__msg__JointCommand {

                unsafe { pendulum_msgs__msg__JointCommand__create() }

            }

            fn destroy_msg(msg: *mut pendulum_msgs__msg__JointCommand) -> () {

                unsafe { pendulum_msgs__msg__JointCommand__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> JointCommand {
  JointCommand {
position: msg.position,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.position = self.position;
}



        }


                          
                          impl Default for JointCommand {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<JointCommand>::new();
                                  JointCommand::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct JointState {

                              pub position: f64,
pub velocity: f64,
pub effort: f64,

                          }

                          impl WrappedTypesupport for JointState { 

            type CStruct = pendulum_msgs__msg__JointState; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pendulum_msgs__msg__JointState() }
            }

            fn create_msg() -> *mut pendulum_msgs__msg__JointState {

                unsafe { pendulum_msgs__msg__JointState__create() }

            }

            fn destroy_msg(msg: *mut pendulum_msgs__msg__JointState) -> () {

                unsafe { pendulum_msgs__msg__JointState__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> JointState {
  JointState {
position: msg.position,
velocity: msg.velocity,
effort: msg.effort,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.position = self.position;
msg.velocity = self.velocity;
msg.effort = self.effort;
}



        }


                          
                          impl Default for JointState {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<JointState>::new();
                                  JointState::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct RttestResults {

                              pub stamp: builtin_interfaces::msg::Time,
pub command: pendulum_msgs::msg::JointCommand,
pub state: pendulum_msgs::msg::JointState,
pub cur_latency: u64,
pub mean_latency: f64,
pub min_latency: u64,
pub max_latency: u64,
pub minor_pagefaults: u64,
pub major_pagefaults: u64,

                          }

                          impl WrappedTypesupport for RttestResults { 

            type CStruct = pendulum_msgs__msg__RttestResults; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__pendulum_msgs__msg__RttestResults() }
            }

            fn create_msg() -> *mut pendulum_msgs__msg__RttestResults {

                unsafe { pendulum_msgs__msg__RttestResults__create() }

            }

            fn destroy_msg(msg: *mut pendulum_msgs__msg__RttestResults) -> () {

                unsafe { pendulum_msgs__msg__RttestResults__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> RttestResults {
  RttestResults {
stamp: builtin_interfaces::msg::Time::from_native(&msg.stamp),
command: pendulum_msgs::msg::JointCommand::from_native(&msg.command),
state: pendulum_msgs::msg::JointState::from_native(&msg.state),
cur_latency: msg.cur_latency,
mean_latency: msg.mean_latency,
min_latency: msg.min_latency,
max_latency: msg.max_latency,
minor_pagefaults: msg.minor_pagefaults,
major_pagefaults: msg.major_pagefaults,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.stamp.copy_to_native(&mut msg.stamp);
self.command.copy_to_native(&mut msg.command);
self.state.copy_to_native(&mut msg.state);
msg.cur_latency = self.cur_latency;
msg.mean_latency = self.mean_latency;
msg.min_latency = self.min_latency;
msg.max_latency = self.max_latency;
msg.minor_pagefaults = self.minor_pagefaults;
msg.major_pagefaults = self.major_pagefaults;
}



        }


                          
                          impl Default for RttestResults {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<RttestResults>::new();
                                  RttestResults::from_native(&msg_native)
                              }
                          }
             


                      }
