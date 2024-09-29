use yew::prelude::*;

// Events
enum Msg {
    addOne,
    substractOne,
}

// state
struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count:0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg:Self::Message) -> bool {
        match msg {
            Msg::addOne => {
                self.count += 1;
                true // re render component
            }
            Msg::substractOne => {
                if self.count>0 {
                    self.count -= 1;
                }
                true   
            }
        
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="container">
                <p>{self.count}</p>
                <button onclick={link.callback(|_| Msg::addOne)}>{"+1"}</button>
                <button onclick={link.callback(|_| Msg::substractOne)}>{"-1"}</button>
            </div>
        }
    }

}


fn main() {
    yew::start_app::<CounterComponent>();
}
