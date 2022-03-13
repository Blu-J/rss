use ammonia::Builder;
use maud::{html, Markup, PreEscaped, DOCTYPE};

pub const TAG_PREFERENCE: &str = "tags";
pub fn ammonia(s: &str) -> String {
    Builder::default()
        .set_tag_attribute_value("img", "loading", "lazy")
        .clean(s)
        .to_string()
}

pub fn with_full_page(title: Markup, body: Markup) -> Markup {
    html! {
        html {
            (DOCTYPE)
            head {
                meta http-equiv="Content-Type" content="text/html" charset="UTF-8";
                title { "RSS" }
                meta name="referrer" content="origin";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" href="/favicon.ico";

                link rel="stylesheet" href="/static/bahunya.css";
                link rel="stylesheet" href="/static/style.css";
                script async[true] src="/static/htmx.min.js" {};

            }
            body hx-boost="true" {
                header {
                    nav {
                        h1 {
                            a href="/" {
                                "Articles"
                            }
                        }
                        input type="checkbox" id="menu-toggle";
                        label for="menu-toggle"{
                            (PreEscaped("&#9776;"))
                        }
                        a href="/sites" { "Sites" }
                        a href="/tags" { "tags" }
                    }
                    (title)
                }
                main {
                        (body)
                    }
                }
            }
    }
}
// fn item<'a>(
//     item: &'a dto::Item,
//     subscription: &'a dto::UserSubscription,
//     show_expanded: bool,
// ) -> Markup {
//     let expanded: Option<Markup> = if show_expanded {
//         Some(html! {
//             (item.description.as_ref().map(|description| html!{
//                 ."article__description shadowed padded"{
//                     (PreEscaped(ammonia(description)))
//                 }
//             }))
//             (item.content.as_ref().map(|content| html!{
//                 ."article__content shadowed padded"{
//                     (PreEscaped(ammonia(content)))
//                 }
//             }))

//         })
//     } else {
//         None
//     };
//     html! {
//         ."article margins-off shadowed padded" id=(format!("article-{}", item.id)) hx-swap="outerHTML" hx-get=(format!("/items/partial/{}",item.id)) {
//             ."article__subscription ellipsis" {
//                 (case::capitalize(&subscription.title, false))
//             }
//             ."article__comments"{
//                 @match &item.comments {
//                     Some(comments) => {
//                         a href=(comments.clone()) {
//                             "Comments"
//                         }
//                     }
//                     None => {
//                         "N/A"
//                     }
//                 }
//             }
//             ."article__title margin ellipsis" {
//                 a href=(item.link.clone()) {
//                     (case::capitalize(&item.title, true))
//                 }
//             }

//         }
//     }
// }
// fn all_subscriptions<'a>(
//     latest_read: i64,
//     subscriptions: Vec<&'a dto::UserSubscription>,
//     subscription_map: HashMap<i64, &'a dto::UserSubscription>,
//     subscriptions_read: HashMap<i64, String>,
//     items: Vec<&'a dto::Item>,
//     sidebar_collapsed: bool,
//     show_unreads: ShowUnreads,
// ) -> Markup {
//     html! {
//         #"all-subscriptions"."margins-off" style=r#"display: grid;
//         background-color: lightgrey;
//         grid-template-columns: 15em auto;"#{
//             @if !sidebar_collapsed {
//                 div#"sidebar"[class="padded", style = r#"
//                 background-color: #294973;
//                 color: #d7dde4;
//                 overflow-y:auto;
//                 overflow-x:hidden;"#, "hx-boost"="true"] {
//                     a[href="/actions/collapse_sidebar"] {
//                         "🍔"
//                     }
//                     div[class="header"] {
//                         input[type="text", placeholder="Search", autocomplete="off"]{}
//                     }
//                     div[class="subscriptions"] {

//                         div[class="subscription_category"] {
//                             a[href="/forms/rss/subscription"]{
//                                 "New"
//                             }
//                             a[href="/actions/filter_all_subscriptions"]{
//                                 "All"
//                             }
//                             @items.len()
//                             @for subscription in subscriptions {
//                                 div[class="subscription_category"] {

//                                     a[href=format!("/actions/filter_by_category/{}", subscription.id)]{
//                                         case::capitalize(&subscription.title, true)
//                                     }
//                                     case::capitalize(subscriptions_read.get(&subscription.id).unwrap_or(&"?".to_string()), true)
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             div#"articles"[class="padded",style=format!(r#"
//             max-height: 100%;
//             overflow-y: auto;
//             overflow-x: hidden;
//             grid-column-end: 5;
//             {}
//             "#, if *sidebar_collapsed {"grid-column-start:1;"}else {""})]{
//                 @if *sidebar_collapsed {
//                     a[href="/actions/expand_sidebar", "hx-boost"="true"]{
//                         "🍔"
//                     }
//                 }
//                 form[action=format!("/actions/mark_all_read/{}", latest_read), method="get", "hx-boost"="true", "hx-push-url"="true"] {
//                     button[type="submit"] {
//                         "Mark All as Read"
//                     }
//                 }
//                 @match show_unreads {
//                     ShowUnreads::ShowEverything => {
//                         form[action="/actions/show_unreads", method="get", "hx-boost"="true", "hx-push-url"="true"] {
//                             button[type="submit"] {
//                                 "Show Unreads"
//                             }
//                         }
//                     }
//                     ShowUnreads::ShowUnreads => {
//                         form[action="/actions/show_everything", method="get", "hx-boost"="true", "hx-push-url"="true"] {
//                             button[type="submit"] {
//                                 "Show Everything"
//                             }
//                         }
//                     }
//                 }
//                 @for item in items {
//                     @let subscription = subscription_map.get(&item.subscription_id).unwrap_or_else(||panic!("{}",item.subscription_id));
//                     @let show_expanded =false;
//                     @Item{
//                         item,
//                         subscription,
//                         show_expanded
//                     }
//                 }
//             }
//         }
//     }
// }
// fn login() -> Markup {
//     html! {
//         form[action="login",method="post","hx-boost"="true","hx-push-url"="true"] {
//             div[class="container"] {
//                 label[for="username"]{
//                     b{
//                         "Username"
//                     }
//                 }
//                 input[type="text",placeholder="Enter Username",name="username",required=true]{}
//                 label[for="password"]{
//                     b{
//                         "Password"
//                     }
//                 }
//                 input[type="password",placeholder="Enter Username",name="password",required=true]{}

//                 button[type="submit"]{
//                     "Login"
//                 }
//             }
//         }
//     }
// }
// fn Subscribe() -> Markup {
//     html! {
//         form[action="/rss/subscriptions",method="post","hx-boost"="true","hx-push-url"="true"] {
//             div[class="container"] {
//                 label[for="category"]{
//                     b{
//                         "Category"
//                     }
//                 }
//                 input[type="text",placeholder="Enter Category",name="category",required=true]{}

//                 label[for="title"]{
//                     b{
//                         "Title"
//                     }
//                 }
//                 input[type="text",placeholder="Enter Title",name="title",required=true]{}

//                 label[for="url"]{
//                     b{
//                         "Url"
//                     }
//                 }
//                 input[type="text",placeholder="Enter Url",name="url",required=true]{}

//                 button[type="submit"]{
//                     "Add New Subscription"
//                 }
//             }
//         }
//     }
// }
