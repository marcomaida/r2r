  pub mod srv {
#[allow(non_snake_case)]
    pub mod AddTwoInts {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__example_interfaces__srv__AddTwoInts()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub a: i64,
pub b: i64,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = example_interfaces__srv__AddTwoInts_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__srv__AddTwoInts_Request() }
            }

            fn create_msg() -> *mut example_interfaces__srv__AddTwoInts_Request {

                unsafe { example_interfaces__srv__AddTwoInts_Request__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__srv__AddTwoInts_Request) -> () {

                unsafe { example_interfaces__srv__AddTwoInts_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
a: msg.a,
b: msg.b,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.a = self.a;
msg.b = self.b;
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub sum: i64,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = example_interfaces__srv__AddTwoInts_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__srv__AddTwoInts_Response() }
            }

            fn create_msg() -> *mut example_interfaces__srv__AddTwoInts_Response {

                unsafe { example_interfaces__srv__AddTwoInts_Response__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__srv__AddTwoInts_Response) -> () {

                unsafe { example_interfaces__srv__AddTwoInts_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
sum: msg.sum,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.sum = self.sum;
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod SetBool {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__example_interfaces__srv__SetBool()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub data: bool,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = example_interfaces__srv__SetBool_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__srv__SetBool_Request() }
            }

            fn create_msg() -> *mut example_interfaces__srv__SetBool_Request {

                unsafe { example_interfaces__srv__SetBool_Request__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__srv__SetBool_Request) -> () {

                unsafe { example_interfaces__srv__SetBool_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub success: bool,
pub message: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = example_interfaces__srv__SetBool_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__srv__SetBool_Response() }
            }

            fn create_msg() -> *mut example_interfaces__srv__SetBool_Response {

                unsafe { example_interfaces__srv__SetBool_Response__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__srv__SetBool_Response) -> () {

                unsafe { example_interfaces__srv__SetBool_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
message: msg.message.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
msg.message.assign(&self.message);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod Trigger {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__example_interfaces__srv__Trigger()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = example_interfaces__srv__Trigger_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__srv__Trigger_Request() }
            }

            fn create_msg() -> *mut example_interfaces__srv__Trigger_Request {

                unsafe { example_interfaces__srv__Trigger_Request__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__srv__Trigger_Request) -> () {

                unsafe { example_interfaces__srv__Trigger_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub success: bool,
pub message: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = example_interfaces__srv__Trigger_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__srv__Trigger_Response() }
            }

            fn create_msg() -> *mut example_interfaces__srv__Trigger_Response {

                unsafe { example_interfaces__srv__Trigger_Response__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__srv__Trigger_Response) -> () {

                unsafe { example_interfaces__srv__Trigger_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
message: msg.message.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
msg.message.assign(&self.message);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
  }
  pub mod action {
#[allow(non_snake_case)]
    pub mod Fibonacci {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Action();
        impl WrappedActionTypeSupport for Action {
            type Goal = Goal;
            type Result = Result;
            type Feedback = Feedback;

            // internal structs
            type FeedbackMessage = FeedbackMessage;
            type SendGoal = SendGoal::Service;
            type GetResult = GetResult::Service;

            fn get_ts() -> &'static rosidl_action_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_action_type_support_handle__example_interfaces__action__Fibonacci()
                }
            }

            fn make_goal_request_msg(goal_id: unique_identifier_msgs::msg::UUID, goal: Goal) -> SendGoal::Request {
                SendGoal::Request {
                     goal_id,
                     goal
                }
            }

            fn make_goal_response_msg(accepted: bool, stamp: builtin_interfaces::msg::Time) -> SendGoal::Response {
                SendGoal::Response {
                     accepted,
                     stamp
                }
            }

            fn make_feedback_msg(goal_id: unique_identifier_msgs::msg::UUID, feedback: Feedback) -> FeedbackMessage {
                FeedbackMessage {
                     goal_id,
                     feedback
                }
            }

            fn make_result_request_msg(goal_id: unique_identifier_msgs::msg::UUID) -> GetResult::Request {
                GetResult::Request {
                     goal_id,
                }
            }

            fn make_result_response_msg(status: i8, result: Result) -> GetResult::Response {
                GetResult::Response {
                     status,
                     result,
                }
            }

            fn destructure_goal_request_msg(msg: SendGoal::Request) -> (unique_identifier_msgs::msg::UUID, Goal) {
                (msg.goal_id, msg.goal)
            }

            fn destructure_goal_response_msg(msg: SendGoal::Response) -> (bool, builtin_interfaces::msg::Time) {
                (msg.accepted, msg.stamp)
            }

            fn destructure_feedback_msg(msg: FeedbackMessage) -> (unique_identifier_msgs::msg::UUID, Feedback) {
                (msg.goal_id, msg.feedback)
            }

            fn destructure_result_response_msg(msg: GetResult::Response) -> (i8, Result) {
                (msg.status, msg.result)
            }

            fn destructure_result_request_msg(msg: GetResult::Request) -> unique_identifier_msgs::msg::UUID {
                msg.goal_id
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Goal {

                              pub order: i32,

                          }

                          impl WrappedTypesupport for Goal { 

            type CStruct = example_interfaces__action__Fibonacci_Goal; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__action__Fibonacci_Goal() }
            }

            fn create_msg() -> *mut example_interfaces__action__Fibonacci_Goal {

                unsafe { example_interfaces__action__Fibonacci_Goal__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__action__Fibonacci_Goal) -> () {

                unsafe { example_interfaces__action__Fibonacci_Goal__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Goal {
  Goal {
order: msg.order,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.order = self.order;
}



        }


                          
                          impl Default for Goal {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Goal>::new();
                                  Goal::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Result {

                              pub sequence: Vec<i32>,

                          }

                          impl WrappedTypesupport for Result { 

            type CStruct = example_interfaces__action__Fibonacci_Result; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__action__Fibonacci_Result() }
            }

            fn create_msg() -> *mut example_interfaces__action__Fibonacci_Result {

                unsafe { example_interfaces__action__Fibonacci_Result__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__action__Fibonacci_Result) -> () {

                unsafe { example_interfaces__action__Fibonacci_Result__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Result {
  Result {
// is_upper_bound_: false
// member.array_size_ : 0
sequence: msg.sequence.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.sequence.update(&self.sequence);
}



        }


                          
                          impl Default for Result {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Result>::new();
                                  Result::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Feedback {

                              pub sequence: Vec<i32>,

                          }

                          impl WrappedTypesupport for Feedback { 

            type CStruct = example_interfaces__action__Fibonacci_Feedback; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__action__Fibonacci_Feedback() }
            }

            fn create_msg() -> *mut example_interfaces__action__Fibonacci_Feedback {

                unsafe { example_interfaces__action__Fibonacci_Feedback__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__action__Fibonacci_Feedback) -> () {

                unsafe { example_interfaces__action__Fibonacci_Feedback__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Feedback {
  Feedback {
// is_upper_bound_: false
// member.array_size_ : 0
sequence: msg.sequence.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.sequence.update(&self.sequence);
}



        }


                          
                          impl Default for Feedback {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Feedback>::new();
                                  Feedback::from_native(&msg_native)
                              }
                          }
             


                    #[allow(non_snake_case)]
    pub mod SendGoal {
    use super::super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__example_interfaces__action__Fibonacci_SendGoal()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub goal_id: unique_identifier_msgs::msg::UUID,
pub goal: example_interfaces::action::Fibonacci::Goal,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = example_interfaces__action__Fibonacci_SendGoal_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__action__Fibonacci_SendGoal_Request() }
            }

            fn create_msg() -> *mut example_interfaces__action__Fibonacci_SendGoal_Request {

                unsafe { example_interfaces__action__Fibonacci_SendGoal_Request__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__action__Fibonacci_SendGoal_Request) -> () {

                unsafe { example_interfaces__action__Fibonacci_SendGoal_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
goal_id: unique_identifier_msgs::msg::UUID::from_native(&msg.goal_id),
goal: example_interfaces::action::Fibonacci::Goal::from_native(&msg.goal),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.goal_id.copy_to_native(&mut msg.goal_id);
self.goal.copy_to_native(&mut msg.goal);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub accepted: bool,
pub stamp: builtin_interfaces::msg::Time,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = example_interfaces__action__Fibonacci_SendGoal_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__action__Fibonacci_SendGoal_Response() }
            }

            fn create_msg() -> *mut example_interfaces__action__Fibonacci_SendGoal_Response {

                unsafe { example_interfaces__action__Fibonacci_SendGoal_Response__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__action__Fibonacci_SendGoal_Response) -> () {

                unsafe { example_interfaces__action__Fibonacci_SendGoal_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
accepted: msg.accepted,
stamp: builtin_interfaces::msg::Time::from_native(&msg.stamp),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.accepted = self.accepted;
self.stamp.copy_to_native(&mut msg.stamp);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod GetResult {
    use super::super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__example_interfaces__action__Fibonacci_GetResult()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub goal_id: unique_identifier_msgs::msg::UUID,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = example_interfaces__action__Fibonacci_GetResult_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__action__Fibonacci_GetResult_Request() }
            }

            fn create_msg() -> *mut example_interfaces__action__Fibonacci_GetResult_Request {

                unsafe { example_interfaces__action__Fibonacci_GetResult_Request__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__action__Fibonacci_GetResult_Request) -> () {

                unsafe { example_interfaces__action__Fibonacci_GetResult_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
goal_id: unique_identifier_msgs::msg::UUID::from_native(&msg.goal_id),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.goal_id.copy_to_native(&mut msg.goal_id);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub status: i8,
pub result: example_interfaces::action::Fibonacci::Result,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = example_interfaces__action__Fibonacci_GetResult_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__action__Fibonacci_GetResult_Response() }
            }

            fn create_msg() -> *mut example_interfaces__action__Fibonacci_GetResult_Response {

                unsafe { example_interfaces__action__Fibonacci_GetResult_Response__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__action__Fibonacci_GetResult_Response) -> () {

                unsafe { example_interfaces__action__Fibonacci_GetResult_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
status: msg.status,
result: example_interfaces::action::Fibonacci::Result::from_native(&msg.result),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.status = self.status;
self.result.copy_to_native(&mut msg.result);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct FeedbackMessage {

                              pub goal_id: unique_identifier_msgs::msg::UUID,
pub feedback: example_interfaces::action::Fibonacci::Feedback,

                          }

                          impl WrappedTypesupport for FeedbackMessage { 

            type CStruct = example_interfaces__action__Fibonacci_FeedbackMessage; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__action__Fibonacci_FeedbackMessage() }
            }

            fn create_msg() -> *mut example_interfaces__action__Fibonacci_FeedbackMessage {

                unsafe { example_interfaces__action__Fibonacci_FeedbackMessage__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__action__Fibonacci_FeedbackMessage) -> () {

                unsafe { example_interfaces__action__Fibonacci_FeedbackMessage__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> FeedbackMessage {
  FeedbackMessage {
goal_id: unique_identifier_msgs::msg::UUID::from_native(&msg.goal_id),
feedback: example_interfaces::action::Fibonacci::Feedback::from_native(&msg.feedback),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.goal_id.copy_to_native(&mut msg.goal_id);
self.feedback.copy_to_native(&mut msg.feedback);
}



        }


                          
                          impl Default for FeedbackMessage {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<FeedbackMessage>::new();
                                  FeedbackMessage::from_native(&msg_native)
                              }
                          }
             


                        }
  }
  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Bool {

                              pub data: bool,

                          }

                          impl WrappedTypesupport for Bool { 

            type CStruct = example_interfaces__msg__Bool; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Bool() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Bool {

                unsafe { example_interfaces__msg__Bool__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Bool) -> () {

                unsafe { example_interfaces__msg__Bool__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Bool {
  Bool {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Bool {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Bool>::new();
                                  Bool::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Byte {

                              pub data: u8,

                          }

                          impl WrappedTypesupport for Byte { 

            type CStruct = example_interfaces__msg__Byte; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Byte() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Byte {

                unsafe { example_interfaces__msg__Byte__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Byte) -> () {

                unsafe { example_interfaces__msg__Byte__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Byte {
  Byte {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Byte {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Byte>::new();
                                  Byte::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ByteMultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<u8>,

                          }

                          impl WrappedTypesupport for ByteMultiArray { 

            type CStruct = example_interfaces__msg__ByteMultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__ByteMultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__ByteMultiArray {

                unsafe { example_interfaces__msg__ByteMultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__ByteMultiArray) -> () {

                unsafe { example_interfaces__msg__ByteMultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ByteMultiArray {
  ByteMultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for ByteMultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ByteMultiArray>::new();
                                  ByteMultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Char {

                              pub data: u8,

                          }

                          impl WrappedTypesupport for Char { 

            type CStruct = example_interfaces__msg__Char; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Char() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Char {

                unsafe { example_interfaces__msg__Char__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Char) -> () {

                unsafe { example_interfaces__msg__Char__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Char {
  Char {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Char {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Char>::new();
                                  Char::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Empty {

                              
                          }

                          impl WrappedTypesupport for Empty { 

            type CStruct = example_interfaces__msg__Empty; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Empty() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Empty {

                unsafe { example_interfaces__msg__Empty__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Empty) -> () {

                unsafe { example_interfaces__msg__Empty__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Empty {
  Empty {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Empty {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Empty>::new();
                                  Empty::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Float32 {

                              pub data: f32,

                          }

                          impl WrappedTypesupport for Float32 { 

            type CStruct = example_interfaces__msg__Float32; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Float32() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Float32 {

                unsafe { example_interfaces__msg__Float32__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Float32) -> () {

                unsafe { example_interfaces__msg__Float32__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Float32 {
  Float32 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Float32 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Float32>::new();
                                  Float32::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Float32MultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<f32>,

                          }

                          impl WrappedTypesupport for Float32MultiArray { 

            type CStruct = example_interfaces__msg__Float32MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Float32MultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Float32MultiArray {

                unsafe { example_interfaces__msg__Float32MultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Float32MultiArray) -> () {

                unsafe { example_interfaces__msg__Float32MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Float32MultiArray {
  Float32MultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Float32MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Float32MultiArray>::new();
                                  Float32MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Float64 {

                              pub data: f64,

                          }

                          impl WrappedTypesupport for Float64 { 

            type CStruct = example_interfaces__msg__Float64; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Float64() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Float64 {

                unsafe { example_interfaces__msg__Float64__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Float64) -> () {

                unsafe { example_interfaces__msg__Float64__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Float64 {
  Float64 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Float64 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Float64>::new();
                                  Float64::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Float64MultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<f64>,

                          }

                          impl WrappedTypesupport for Float64MultiArray { 

            type CStruct = example_interfaces__msg__Float64MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Float64MultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Float64MultiArray {

                unsafe { example_interfaces__msg__Float64MultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Float64MultiArray) -> () {

                unsafe { example_interfaces__msg__Float64MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Float64MultiArray {
  Float64MultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Float64MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Float64MultiArray>::new();
                                  Float64MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int16 {

                              pub data: i16,

                          }

                          impl WrappedTypesupport for Int16 { 

            type CStruct = example_interfaces__msg__Int16; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Int16() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Int16 {

                unsafe { example_interfaces__msg__Int16__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Int16) -> () {

                unsafe { example_interfaces__msg__Int16__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int16 {
  Int16 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Int16 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int16>::new();
                                  Int16::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int16MultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<i16>,

                          }

                          impl WrappedTypesupport for Int16MultiArray { 

            type CStruct = example_interfaces__msg__Int16MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Int16MultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Int16MultiArray {

                unsafe { example_interfaces__msg__Int16MultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Int16MultiArray) -> () {

                unsafe { example_interfaces__msg__Int16MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int16MultiArray {
  Int16MultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Int16MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int16MultiArray>::new();
                                  Int16MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int32 {

                              pub data: i32,

                          }

                          impl WrappedTypesupport for Int32 { 

            type CStruct = example_interfaces__msg__Int32; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Int32() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Int32 {

                unsafe { example_interfaces__msg__Int32__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Int32) -> () {

                unsafe { example_interfaces__msg__Int32__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int32 {
  Int32 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Int32 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int32>::new();
                                  Int32::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int32MultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<i32>,

                          }

                          impl WrappedTypesupport for Int32MultiArray { 

            type CStruct = example_interfaces__msg__Int32MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Int32MultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Int32MultiArray {

                unsafe { example_interfaces__msg__Int32MultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Int32MultiArray) -> () {

                unsafe { example_interfaces__msg__Int32MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int32MultiArray {
  Int32MultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Int32MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int32MultiArray>::new();
                                  Int32MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int64 {

                              pub data: i64,

                          }

                          impl WrappedTypesupport for Int64 { 

            type CStruct = example_interfaces__msg__Int64; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Int64() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Int64 {

                unsafe { example_interfaces__msg__Int64__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Int64) -> () {

                unsafe { example_interfaces__msg__Int64__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int64 {
  Int64 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Int64 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int64>::new();
                                  Int64::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int64MultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<i64>,

                          }

                          impl WrappedTypesupport for Int64MultiArray { 

            type CStruct = example_interfaces__msg__Int64MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Int64MultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Int64MultiArray {

                unsafe { example_interfaces__msg__Int64MultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Int64MultiArray) -> () {

                unsafe { example_interfaces__msg__Int64MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int64MultiArray {
  Int64MultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Int64MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int64MultiArray>::new();
                                  Int64MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int8 {

                              pub data: i8,

                          }

                          impl WrappedTypesupport for Int8 { 

            type CStruct = example_interfaces__msg__Int8; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Int8() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Int8 {

                unsafe { example_interfaces__msg__Int8__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Int8) -> () {

                unsafe { example_interfaces__msg__Int8__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int8 {
  Int8 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for Int8 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int8>::new();
                                  Int8::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Int8MultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<i8>,

                          }

                          impl WrappedTypesupport for Int8MultiArray { 

            type CStruct = example_interfaces__msg__Int8MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__Int8MultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__Int8MultiArray {

                unsafe { example_interfaces__msg__Int8MultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__Int8MultiArray) -> () {

                unsafe { example_interfaces__msg__Int8MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Int8MultiArray {
  Int8MultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for Int8MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Int8MultiArray>::new();
                                  Int8MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MultiArrayDimension {

                              pub label: std::string::String,
pub size: u32,
pub stride: u32,

                          }

                          impl WrappedTypesupport for MultiArrayDimension { 

            type CStruct = example_interfaces__msg__MultiArrayDimension; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__MultiArrayDimension() }
            }

            fn create_msg() -> *mut example_interfaces__msg__MultiArrayDimension {

                unsafe { example_interfaces__msg__MultiArrayDimension__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__MultiArrayDimension) -> () {

                unsafe { example_interfaces__msg__MultiArrayDimension__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MultiArrayDimension {
  MultiArrayDimension {
label: msg.label.to_str().to_owned(),
size: msg.size,
stride: msg.stride,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.label.assign(&self.label);
msg.size = self.size;
msg.stride = self.stride;
}



        }


                          
                          impl Default for MultiArrayDimension {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MultiArrayDimension>::new();
                                  MultiArrayDimension::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MultiArrayLayout {

                              pub dim: Vec<example_interfaces::msg::MultiArrayDimension>,
pub data_offset: u32,

                          }

                          impl WrappedTypesupport for MultiArrayLayout { 

            type CStruct = example_interfaces__msg__MultiArrayLayout; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__MultiArrayLayout() }
            }

            fn create_msg() -> *mut example_interfaces__msg__MultiArrayLayout {

                unsafe { example_interfaces__msg__MultiArrayLayout__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__MultiArrayLayout) -> () {

                unsafe { example_interfaces__msg__MultiArrayLayout__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MultiArrayLayout {
  MultiArrayLayout {
// is_upper_bound_: false
// member.array_size_ : 0
dim : {
let mut temp = Vec::with_capacity(msg.dim.size);
let slice = unsafe { std::slice::from_raw_parts(msg.dim.data, msg.dim.size)};
for s in slice { temp.push(example_interfaces::msg::MultiArrayDimension::from_native(s)); }
temp },
data_offset: msg.data_offset,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { example_interfaces__msg__MultiArrayDimension__Sequence__fini(&mut msg.dim) };
unsafe { example_interfaces__msg__MultiArrayDimension__Sequence__init(&mut msg.dim, self.dim.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.dim.data, msg.dim.size)};
for (t,s) in slice.iter_mut().zip(&self.dim) { s.copy_to_native(t);}
msg.data_offset = self.data_offset;
}



        }


                          
                          impl Default for MultiArrayLayout {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MultiArrayLayout>::new();
                                  MultiArrayLayout::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct String {

                              pub data: std::string::String,

                          }

                          impl WrappedTypesupport for String { 

            type CStruct = example_interfaces__msg__String; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__String() }
            }

            fn create_msg() -> *mut example_interfaces__msg__String {

                unsafe { example_interfaces__msg__String__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__String) -> () {

                unsafe { example_interfaces__msg__String__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> String {
  String {
data: msg.data.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data.assign(&self.data);
}



        }


                          
                          impl Default for String {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<String>::new();
                                  String::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt16 {

                              pub data: u16,

                          }

                          impl WrappedTypesupport for UInt16 { 

            type CStruct = example_interfaces__msg__UInt16; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__UInt16() }
            }

            fn create_msg() -> *mut example_interfaces__msg__UInt16 {

                unsafe { example_interfaces__msg__UInt16__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__UInt16) -> () {

                unsafe { example_interfaces__msg__UInt16__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt16 {
  UInt16 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for UInt16 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt16>::new();
                                  UInt16::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt16MultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<u16>,

                          }

                          impl WrappedTypesupport for UInt16MultiArray { 

            type CStruct = example_interfaces__msg__UInt16MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__UInt16MultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__UInt16MultiArray {

                unsafe { example_interfaces__msg__UInt16MultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__UInt16MultiArray) -> () {

                unsafe { example_interfaces__msg__UInt16MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt16MultiArray {
  UInt16MultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for UInt16MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt16MultiArray>::new();
                                  UInt16MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt32 {

                              pub data: u32,

                          }

                          impl WrappedTypesupport for UInt32 { 

            type CStruct = example_interfaces__msg__UInt32; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__UInt32() }
            }

            fn create_msg() -> *mut example_interfaces__msg__UInt32 {

                unsafe { example_interfaces__msg__UInt32__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__UInt32) -> () {

                unsafe { example_interfaces__msg__UInt32__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt32 {
  UInt32 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for UInt32 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt32>::new();
                                  UInt32::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt32MultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<u32>,

                          }

                          impl WrappedTypesupport for UInt32MultiArray { 

            type CStruct = example_interfaces__msg__UInt32MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__UInt32MultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__UInt32MultiArray {

                unsafe { example_interfaces__msg__UInt32MultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__UInt32MultiArray) -> () {

                unsafe { example_interfaces__msg__UInt32MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt32MultiArray {
  UInt32MultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for UInt32MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt32MultiArray>::new();
                                  UInt32MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt64 {

                              pub data: u64,

                          }

                          impl WrappedTypesupport for UInt64 { 

            type CStruct = example_interfaces__msg__UInt64; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__UInt64() }
            }

            fn create_msg() -> *mut example_interfaces__msg__UInt64 {

                unsafe { example_interfaces__msg__UInt64__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__UInt64) -> () {

                unsafe { example_interfaces__msg__UInt64__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt64 {
  UInt64 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for UInt64 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt64>::new();
                                  UInt64::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt64MultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<u64>,

                          }

                          impl WrappedTypesupport for UInt64MultiArray { 

            type CStruct = example_interfaces__msg__UInt64MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__UInt64MultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__UInt64MultiArray {

                unsafe { example_interfaces__msg__UInt64MultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__UInt64MultiArray) -> () {

                unsafe { example_interfaces__msg__UInt64MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt64MultiArray {
  UInt64MultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for UInt64MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt64MultiArray>::new();
                                  UInt64MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt8 {

                              pub data: u8,

                          }

                          impl WrappedTypesupport for UInt8 { 

            type CStruct = example_interfaces__msg__UInt8; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__UInt8() }
            }

            fn create_msg() -> *mut example_interfaces__msg__UInt8 {

                unsafe { example_interfaces__msg__UInt8__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__UInt8) -> () {

                unsafe { example_interfaces__msg__UInt8__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt8 {
  UInt8 {
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data = self.data;
}



        }


                          
                          impl Default for UInt8 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt8>::new();
                                  UInt8::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct UInt8MultiArray {

                              pub layout: example_interfaces::msg::MultiArrayLayout,
pub data: Vec<u8>,

                          }

                          impl WrappedTypesupport for UInt8MultiArray { 

            type CStruct = example_interfaces__msg__UInt8MultiArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__UInt8MultiArray() }
            }

            fn create_msg() -> *mut example_interfaces__msg__UInt8MultiArray {

                unsafe { example_interfaces__msg__UInt8MultiArray__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__UInt8MultiArray) -> () {

                unsafe { example_interfaces__msg__UInt8MultiArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> UInt8MultiArray {
  UInt8MultiArray {
layout: example_interfaces::msg::MultiArrayLayout::from_native(&msg.layout),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.layout.copy_to_native(&mut msg.layout);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for UInt8MultiArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<UInt8MultiArray>::new();
                                  UInt8MultiArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct WString {

                              pub data: std::string::String,

                          }

                          impl WrappedTypesupport for WString { 

            type CStruct = example_interfaces__msg__WString; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__example_interfaces__msg__WString() }
            }

            fn create_msg() -> *mut example_interfaces__msg__WString {

                unsafe { example_interfaces__msg__WString__create() }

            }

            fn destroy_msg(msg: *mut example_interfaces__msg__WString) -> () {

                unsafe { example_interfaces__msg__WString__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> WString {
  WString {
data: msg.data.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data.assign(&self.data);
}



        }


                          
                          impl Default for WString {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<WString>::new();
                                  WString::from_native(&msg_native)
                              }
                          }
             


                      }
