  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Frame {

                              pub header: std_msgs::msg::Header,
pub id: u32,
pub is_rtr: bool,
pub is_extended: bool,
pub is_error: bool,
pub dlc: u8,
pub data: Vec<u8>,

                          }

                          impl WrappedTypesupport for Frame { 

            type CStruct = can_msgs__msg__Frame; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__can_msgs__msg__Frame() }
            }

            fn create_msg() -> *mut can_msgs__msg__Frame {

                unsafe { can_msgs__msg__Frame__create() }

            }

            fn destroy_msg(msg: *mut can_msgs__msg__Frame) -> () {

                unsafe { can_msgs__msg__Frame__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Frame {
  Frame {
header: std_msgs::msg::Header::from_native(&msg.header),
id: msg.id,
is_rtr: msg.is_rtr,
is_extended: msg.is_extended,
is_error: msg.is_error,
dlc: msg.dlc,
// is_upper_bound_: false
// member.array_size_ : 8
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.id = self.id;
msg.is_rtr = self.is_rtr;
msg.is_extended = self.is_extended;
msg.is_error = self.is_error;
msg.dlc = self.dlc;
assert_eq!(self.data.len(), 8, "Field {} is fixed size of {}!", "data", 8);
msg.data.copy_from_slice(&self.data[..8]);
}



        }


                          
                          impl Default for Frame {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Frame>::new();
                                  Frame::from_native(&msg_native)
                              }
                          }
             


                      }
