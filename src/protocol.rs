//! The general interface between expressions and checkers.
//!
//! The implementation follows an abstract client-server relation.
//! Clients are able to send messages to the server, and can also point to other clients.
//! In the case of expressions, these ...

pub type ClientsWithCtx<'a, Ctx, Msg> = Vec<(&'a dyn Client<Ctx = Ctx, Msg = Msg>, Ctx)>;
pub trait Client {
    /// Context that the type may require.
    type Ctx: Copy;
    /// The message type to send to servers.
    type Msg;

    /// Returns a list of other `Client`s "connected" to Self.
    /// The context may change within the network, given the relationship of the `Client`s,
    /// hence the context is returned separately to allow for possible modifications.
    fn children(&self, ctx: Self::Ctx) -> ClientsWithCtx<Self::Ctx, Self::Msg>;

    /// Return all of the messages that Self wants to send to the server.
    /// Should **not** include the messages of any peer `Client`s.
    fn messages(&self, ctx: Self::Ctx) -> Vec<Self::Msg>;

    /// Sends all messages relating to Self.
    /// By default this first sends all the messages of its children (recursively),
    /// before sending the messages from Self.
    fn send_all(&self, ctx: Self::Ctx, server: &mut dyn Server<Msg = Self::Msg>) {
        // send children's messages first
        for (child, ctx) in self.children(ctx) {
            child.send_all(ctx, server);
        }

        // send Self's messages
        for msg in self.messages(ctx) {
            server.accept(msg);
        }
    }
}

pub trait Server {
    /// The message type to receive from clients.
    type Msg;

    /// Receives and processes the given message.
    fn accept(&mut self, msg: Self::Msg);
}

// Separated from `Server` as it may want to use generics.
// If this were part of the normal `Server` definition, these generics
// would need to be specified by the `Client`s, which we don't want.
pub trait ServerHandler: Server {
    /// Returns the server's state
    // TODO: Add proper return types
    #[allow(clippy::result_unit_err)]
    fn state(&self) -> Result<(), ()>;

    /// Resets the server's state
    fn reset(&mut self);
}
