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
                    &*rosidl_typesupport_c__get_action_type_support_handle__action_tutorials_interfaces__action__Fibonacci()
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

            type CStruct = action_tutorials_interfaces__action__Fibonacci_Goal; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_tutorials_interfaces__action__Fibonacci_Goal() }
            }

            fn create_msg() -> *mut action_tutorials_interfaces__action__Fibonacci_Goal {

                unsafe { action_tutorials_interfaces__action__Fibonacci_Goal__create() }

            }

            fn destroy_msg(msg: *mut action_tutorials_interfaces__action__Fibonacci_Goal) -> () {

                unsafe { action_tutorials_interfaces__action__Fibonacci_Goal__destroy(msg) };

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

            type CStruct = action_tutorials_interfaces__action__Fibonacci_Result; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_tutorials_interfaces__action__Fibonacci_Result() }
            }

            fn create_msg() -> *mut action_tutorials_interfaces__action__Fibonacci_Result {

                unsafe { action_tutorials_interfaces__action__Fibonacci_Result__create() }

            }

            fn destroy_msg(msg: *mut action_tutorials_interfaces__action__Fibonacci_Result) -> () {

                unsafe { action_tutorials_interfaces__action__Fibonacci_Result__destroy(msg) };

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

                              pub partial_sequence: Vec<i32>,

                          }

                          impl WrappedTypesupport for Feedback { 

            type CStruct = action_tutorials_interfaces__action__Fibonacci_Feedback; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_tutorials_interfaces__action__Fibonacci_Feedback() }
            }

            fn create_msg() -> *mut action_tutorials_interfaces__action__Fibonacci_Feedback {

                unsafe { action_tutorials_interfaces__action__Fibonacci_Feedback__create() }

            }

            fn destroy_msg(msg: *mut action_tutorials_interfaces__action__Fibonacci_Feedback) -> () {

                unsafe { action_tutorials_interfaces__action__Fibonacci_Feedback__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Feedback {
  Feedback {
// is_upper_bound_: false
// member.array_size_ : 0
partial_sequence: msg.partial_sequence.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.partial_sequence.update(&self.partial_sequence);
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
                    &*rosidl_typesupport_c__get_service_type_support_handle__action_tutorials_interfaces__action__Fibonacci_SendGoal()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub goal_id: unique_identifier_msgs::msg::UUID,
pub goal: action_tutorials_interfaces::action::Fibonacci::Goal,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = action_tutorials_interfaces__action__Fibonacci_SendGoal_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_tutorials_interfaces__action__Fibonacci_SendGoal_Request() }
            }

            fn create_msg() -> *mut action_tutorials_interfaces__action__Fibonacci_SendGoal_Request {

                unsafe { action_tutorials_interfaces__action__Fibonacci_SendGoal_Request__create() }

            }

            fn destroy_msg(msg: *mut action_tutorials_interfaces__action__Fibonacci_SendGoal_Request) -> () {

                unsafe { action_tutorials_interfaces__action__Fibonacci_SendGoal_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
goal_id: unique_identifier_msgs::msg::UUID::from_native(&msg.goal_id),
goal: action_tutorials_interfaces::action::Fibonacci::Goal::from_native(&msg.goal),
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

            type CStruct = action_tutorials_interfaces__action__Fibonacci_SendGoal_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_tutorials_interfaces__action__Fibonacci_SendGoal_Response() }
            }

            fn create_msg() -> *mut action_tutorials_interfaces__action__Fibonacci_SendGoal_Response {

                unsafe { action_tutorials_interfaces__action__Fibonacci_SendGoal_Response__create() }

            }

            fn destroy_msg(msg: *mut action_tutorials_interfaces__action__Fibonacci_SendGoal_Response) -> () {

                unsafe { action_tutorials_interfaces__action__Fibonacci_SendGoal_Response__destroy(msg) };

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
                    &*rosidl_typesupport_c__get_service_type_support_handle__action_tutorials_interfaces__action__Fibonacci_GetResult()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub goal_id: unique_identifier_msgs::msg::UUID,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = action_tutorials_interfaces__action__Fibonacci_GetResult_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_tutorials_interfaces__action__Fibonacci_GetResult_Request() }
            }

            fn create_msg() -> *mut action_tutorials_interfaces__action__Fibonacci_GetResult_Request {

                unsafe { action_tutorials_interfaces__action__Fibonacci_GetResult_Request__create() }

            }

            fn destroy_msg(msg: *mut action_tutorials_interfaces__action__Fibonacci_GetResult_Request) -> () {

                unsafe { action_tutorials_interfaces__action__Fibonacci_GetResult_Request__destroy(msg) };

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
pub result: action_tutorials_interfaces::action::Fibonacci::Result,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = action_tutorials_interfaces__action__Fibonacci_GetResult_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_tutorials_interfaces__action__Fibonacci_GetResult_Response() }
            }

            fn create_msg() -> *mut action_tutorials_interfaces__action__Fibonacci_GetResult_Response {

                unsafe { action_tutorials_interfaces__action__Fibonacci_GetResult_Response__create() }

            }

            fn destroy_msg(msg: *mut action_tutorials_interfaces__action__Fibonacci_GetResult_Response) -> () {

                unsafe { action_tutorials_interfaces__action__Fibonacci_GetResult_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
status: msg.status,
result: action_tutorials_interfaces::action::Fibonacci::Result::from_native(&msg.result),
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
pub feedback: action_tutorials_interfaces::action::Fibonacci::Feedback,

                          }

                          impl WrappedTypesupport for FeedbackMessage { 

            type CStruct = action_tutorials_interfaces__action__Fibonacci_FeedbackMessage; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__action_tutorials_interfaces__action__Fibonacci_FeedbackMessage() }
            }

            fn create_msg() -> *mut action_tutorials_interfaces__action__Fibonacci_FeedbackMessage {

                unsafe { action_tutorials_interfaces__action__Fibonacci_FeedbackMessage__create() }

            }

            fn destroy_msg(msg: *mut action_tutorials_interfaces__action__Fibonacci_FeedbackMessage) -> () {

                unsafe { action_tutorials_interfaces__action__Fibonacci_FeedbackMessage__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> FeedbackMessage {
  FeedbackMessage {
goal_id: unique_identifier_msgs::msg::UUID::from_native(&msg.goal_id),
feedback: action_tutorials_interfaces::action::Fibonacci::Feedback::from_native(&msg.feedback),
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
