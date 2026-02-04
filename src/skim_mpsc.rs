use skim::tui::Event;
use std::sync::mpsc;

pub enum Request {
	CallLuaAction((usize, usize)),
	Done(),
}

pub enum Response {
	Actions(Vec<Event>),
}
pub struct Msg {
	pub data: Request,
	pub reply: Option<mpsc::Sender<Response>>,
}

impl Msg {
	pub fn done(&self, data: Response) -> Result<(), mpsc::SendError<Response>> {
		if let Some(reply) = &self.reply {
			reply.send(data)
		} else {
			Ok(())
		}
	}
}

pub fn server() -> (mpsc::Sender<Msg>, mpsc::Receiver<Msg>) {
	mpsc::channel::<Msg>()
}

pub struct Client {
	pub server: mpsc::Sender<Msg>,
	sender: mpsc::Sender<Response>,
	reciver: mpsc::Receiver<Response>,
}

impl Client {
	pub fn new(server: mpsc::Sender<Msg>) -> Self {
		let (tx, rx) = mpsc::channel::<Response>();
		Client {
			server: server.clone(),
			sender: tx,
			reciver: rx,
		}
	}
	pub fn send(&self, req: Request) -> Option<Response> {
		let msg = Msg {
			data: req,
			reply: Some(self.sender.clone()),
		};
		self.server.send(msg).ok();
		self.reciver.recv().ok()
	}
	pub fn call_lua(&self, lua_ptr: usize, app_ptr: usize) -> Option<Vec<Event>> {
		match self.send(Request::CallLuaAction((lua_ptr, app_ptr))) {
			Some(responses) => match responses {
				Response::Actions(actions) => Some(actions),
			},
			None => None,
		}
	}
}
