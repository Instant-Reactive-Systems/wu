use crate::prelude::*;
use gloo_net::http::Request;
use gloo_timers::future::TimeoutFuture;
use wu::ActionButton;

async fn get_time_from_api(tz : String) -> String {
    match Request::get(format!("https://timeapi.io/api/time/current/zone?timeZone={}", tz).as_str()).send().await {
        Ok(res) => {
            res.text().await.expect("to be serializable")
        },
        Err(err) => {
            format!("Error has occured: {}", err.to_string())
        },
    }
}

#[component]
pub fn ActionButtonDocs() -> impl IntoView {
    let (text_from_fetched_time, set_text_from_fetched_time) = signal(String::from("Response from Time API will be displayed here"));
    let (time_zone, set_time_zone) = signal(String::from("Europe/Zagreb"));

    const ACTION_BUTTON_CODE : &str = 
r##"<ActionButton 
action=fetch_time 
input={move || time_zone.get()}
idle_view={move || {"Get the time"}}
pending_view={move || {"Getting the time"}}
finished_view={move |_fetched_time| {"This is the time"}}
on_finish={move |fetched_time| {set_text_from_fetched_time.set(fetched_time)}}
finished_lasts_for=1000
/>"##;
    
    let fetch_time = Action::new(move |tz : &String| {
        set_text_from_fetched_time.set(String::new());
        let tz = tz.to_owned();
        send_wrapper::SendWrapper::new(async move {
            let time = get_time_from_api(tz).await;
            TimeoutFuture::new(750).await;
            time
        })
    });


    
    view! {
        <p>"ActionButton is a wrapper component of <button> that handles on-click async actions with added functionalities."</p>
        
        <div style="font-family: sans-serif; font-size: 14px; line-height: 1.5;">
            <h3 style="margin-bottom: 0.25em;">Required Props</h3>
            <ul style="list-style: none; padding-left: 0;">
                <li>
                <strong>action</strong>: <code>Action&lt;I, O&gt;</code><br />
                <span style="color: gray; margin-left: 2em;">Action to dispatch and to await.</span>
                </li>
                <li>
                <strong>input</strong>: <code>impl Into&lt;Callback&lt;(), I&gt;&gt;</code><br />
                <span style="color: gray; margin-left: 2em;">Input to the action.</span>
                </li>
                <li>
                <strong>finished_lasts_for</strong>: <code>impl Into&lt;f64&gt;</code><br />
                <span style="color: gray; margin-left: 2em;">How long the finished state will last for.</span>
                </li>
            </ul>

            <h3 style="margin-bottom: 0.25em; margin-top: 1em;">Optional Props</h3>
            <ul style="list-style: none; padding-left: 0;">
                <li>
                <strong>idle_view</strong>: <code>impl Into&lt;ViewFn&gt;</code><br />
                <span style="color: gray; margin-left: 2em;">View to display during idle state.</span>
                </li>
                <li>
                <strong>pending_view</strong>: <code>impl Into&lt;ViewFn&gt;</code><br />
                <span style="color: gray; margin-left: 2em;">View to display during pending state.</span>
                </li>
                <li>
                <strong>finished_view</strong>: <code>impl Into&lt;crate::utils::ViewFnWithArgs&lt;O&gt;&gt;</code><br />
                <span style="color: gray; margin-left: 2em;">View to display during finished state.</span>
                </li>
                <li>
                <strong>on_finish</strong>: <code>impl Into&lt;Callback&lt;(O,), ()&gt;&gt;</code><br />
                <span style="color: gray; margin-left: 2em;">Logic to run after the finished state.</span>
                </li>
            </ul>
        </div>

        <p>"Example used below:"</p>
        <div>
            <p><code>"<ActionButton"</code></p>
            <p><code style="margin-left: 2em;">"action=fetch_time"</code></p>
            <p><code style="margin-left: 2em;">"input={move || time_zone.get()}"</code></p>
            <p><code style="margin-left: 2em;">r##"idle_view={move || {"Get the time"}}"##</code></p>
            <p><code style="margin-left: 2em;">r##"pending_view={move || {"Getting the time"}}"##</code></p>
            <p><code style="margin-left: 2em;">r##"finished_view={move |_fetched_time| {"This is the time"}}"##</code></p>
            <p><code style="margin-left: 2em;">"on_finish={move |fetched_time| {set_text_from_fetched_time.set(fetched_time)}}"</code></p>
            <p><code style="margin-left: 2em;">"finished_lasts_for=1000"</code></p>
            <p><code>"/>"</code></p>
        </div>

        <div>
            <input type="text" bind:value=(time_zone, set_time_zone)/>
        </div>

        <ActionButton 
            action=fetch_time 
            input={move || time_zone.get()}
            idle_view={move || {"Get the time"}}
            pending_view={move || {"Getting the time"}}
            finished_view={move |_fetched_time| {"This is the time"}}
            on_finish={move |fetched_time| {set_text_from_fetched_time.set(fetched_time)}}
            finished_lasts_for=1000
            />
        <div>
            <p>{text_from_fetched_time}</p>
        </div>
    }

}