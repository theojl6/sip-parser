pub struct Sip {
    pub method: SipMethod,
}

pub enum SipMethod {
    Invite,
    Bye,
}
