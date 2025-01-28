use super::action_response::ActionResponse;

pub trait IntoActionResponse {
    fn into_action_response(self) -> ActionResponse;
}