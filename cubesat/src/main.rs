#[allow(unused_variables)]

#[derive(Debug)]
struct CubeSat{
    id: u64,
    mailBox: MailBox,
}

#[derive(Debug)]
struct MailBox{
    messages: Vec<Message>
}

type Message = String;

struct GroundStation{}
impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailBox.messages.push(msg);
    }
}

impl CubeSat {
    fn recv(&mut self) {
        self.mailBox.messages.pop();
    }
}

fn main() {
    let base = GroundStation{};
    let mut sat_a = CubeSat{
        id: 0,
        mailBox: MailBox { messages: vec![] }
    };

    println!("{:?}", sat_a);
    base.send(&mut sat_a, Message::from("Hi!"));
    
    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);

}
