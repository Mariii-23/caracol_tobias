#[macro_export]
macro_rules! cmd_ctx_msg {
    ($name: ident,$($line: stmt)*) => {
        #[command]
        fn $name (ctx: &mut Context, msg: &Message)-> CommandResult{
           $($line)*
           Ok(())
        }
    };
}

#[macro_export]
macro_rules! cmd_ctx_msg_args {
    ($name: ident,$($line: stmt)*) => {
        #[command]
        fn $name (ctx: &mut Context, msg: &Message,args:Args)-> CommandResult{
           $($line)*
           Ok(())
        }
    };
}
