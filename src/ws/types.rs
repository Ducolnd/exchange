use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {

}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {

}

/// Client feels the need to re-receive data, 
/// maybe an error occured on client side
#[derive(Message)]
#[rtype(result = "()")]
pub struct Refresh {

}