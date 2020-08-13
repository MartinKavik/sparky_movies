#![allow(clippy::wildcard_imports)]
// @TODO: Remove.
#![allow(dead_code, unused_variables)]

use seed::{prelude::*, *};

use seed_style::{px, em, pc, rem, Style};
use seed_style::*;

mod page;

const MOVIES: &str = "movies";
const TIME_TRACKER: &str = "time_tracker";
const TIME_BLOCKS: &str = "time_blocks";
const SETTINGS: &str = "settings";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    init_styles();

    orders
        .subscribe(Msg::UrlChanged)
        .stream(streams::window_event(Ev::Click, |_| Msg::HideMenu));

    Model {
        ctx: Context {
            // user: None,
            user: Some(User {
                username: "John".to_owned(),
                email: "john@email.com".to_owned(),
            }),
            token: None,
        },
        base_url: url.to_base_url(),
        page: Page::init(url, orders),
        menu_visible: false,
    }
}

fn init_styles() {
    load_app_themes(&[default_breakpoint_theme, default_scale_theme]);

    GlobalStyle::default()
    .style(
        "body", // @TODO: should be "html" once possible
        s()
            .font_size(px(16))
            .box_sizing(CssBoxSizing::BorderBox)
            .raw("text-rendering: optimizeLegibility;")
            .raw(format!("-webkit-text-size-adjust: {};", pc(100)).as_str())
            .raw(format!("-moz-text-size-adjust: {};", pc(100)).as_str())
    )
    .style(
        "body",
        s()
            .color("#4a4a4a")
            .font_size(em(1))
            .font_weight("400")
            .line_height("1.5")
    )
    .style(
        "body, button, input, select, textarea", 
        s()
            .font_family(r#"BlinkMacSystemFont,-apple-system,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Fira Sans","Droid Sans","Helvetica Neue",Helvetica,Arial,sans-serif"#)
    )
    .style(
        "*, ::after, ::before",
        s()
            .box_sizing(CssBoxSizing::Inherit)
    )
    .style(
        "a",
        s()
            .color("#3273dc")
            .cursor(CssCursor::Pointer)
            .text_decoration(CssTextDecoration::None)
    )
    .style(
        "a",
        s()
            .hover()
            .color("#363636")
    )
    .style(
        "span",
        s()
            .font_style(CssFontStyle::Inherit)
            .font_weight(CssFontWeight::Inherit)
    )
    .style(
        "blockquote, body, dd, dl, dt, fieldset, figure, h1, h2, h3, h4, h5, h6, hr, html, iframe, legend, li, ol, p, pre, textarea, ul",
        s()
            .m("0")
            .p("0")
    )
    .activate_init_styles();
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Breakpoint {
    // basic
    Mobile,
    Tablet,
    Desktop,
    WideScreen,
    FullHD,
    // extra
    TabletOnly,
    Touch,
    DesktopOnly,
    WideScreenOnly,
}
impl BreakpointTheme for Breakpoint {} 

fn default_breakpoint_theme() -> Theme {
    use Breakpoint::*;
    Theme::new("default_breakpoint_theme")
        // basic
        .set_breakpoint(Mobile, (0, Some(769))) 
        .set_breakpoint(Tablet, (769, Some(1024)))
        .set_breakpoint(Desktop, (1024, Some(1216)))
        .set_breakpoint(WideScreen, (1216, Some(1408)))
        .set_breakpoint(FullHD, (1408, None))
        .breakpoint_scale([769, 1024, 1216, 1408]) 
        // extra
        .set_breakpoint(TabletOnly, (769, Some(1024)))
        .set_breakpoint(Touch, (0, Some(1024)))
        .set_breakpoint(DesktopOnly, (1024, Some(1216)))
        .set_breakpoint(WideScreenOnly, (1216, Some(1408)))
}

fn default_scale_theme() -> Theme {
    Theme::new("default_scale_theme")
        .font_size_scale(&[rem(3), rem(2.5), rem(2), rem(1.5), rem(1.25), rem(1), rem(0.75)])
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    base_url: Url,
    page: Page,
    menu_visible: bool,
}

struct Context {
    user: Option<User>,
    token: Option<String>,
}

struct User {
    username: String,
    email: String,
}

// ------ Page ------

enum Page {
    Home,
    Movies(page::movies::Model),
    Settings(page::settings::Model),
    NotFound,
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Self::Home,
            [MOVIES] => Self::Movies(
                page::movies::init(url, &mut orders.proxy(Msg::MoviesMsg))
            ),
            [SETTINGS] => Self::Settings(
                page::settings::init(url, &mut orders.proxy(Msg::SettingsMsg))
            ),
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
    fn movies(self) -> Url {
        self.base_url().add_path_part(MOVIES)
    }
    fn settings(self) -> Url {
        self.base_url().add_path_part(SETTINGS)
    }
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
    ToggleMenu,
    HideMenu,

    // ------ pages ------

    MoviesMsg(page::movies::Msg),
    SettingsMsg(page::settings::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders),
        Msg::ToggleMenu => model.menu_visible = not(model.menu_visible),
        Msg::HideMenu => {
            if model.menu_visible {
                model.menu_visible = false;
            } else {
                orders.skip();
            }
        },

        // ------ pages ------

        Msg::MoviesMsg(msg) => {
            if let Page::Movies(model) = &mut model.page {
                page::movies::update(msg, model, &mut orders.proxy(Msg::MoviesMsg))
            }
        }
        Msg::SettingsMsg(msg) => {
            if let Page::Settings(model) = &mut model.page {
                page::settings::update(msg, model, &mut orders.proxy(Msg::SettingsMsg))
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        view_navbar(model.menu_visible, &model.base_url, model.ctx.user.as_ref(), &model.page),
        view_content(&model.page),
    ]
}

// ----- view_content ------

fn view_content(page: &Page) -> Node<Msg> {
    div![
        C!["container"],
        s()
            .flex_grow("1")
            .my("0")
            .mx("auto")
            .position(CssPosition::Relative)
            .w(CssWidth::Auto),
        s()
            .only(Breakpoint::Desktop)
            .max_w(px(960)),
        s()
            .only(Breakpoint::WideScreen)
            .max_w(px(1152)),
        s()
            .only_and_above(Breakpoint::FullHD)
            .max_w(px(1344)),
        match page {
            Page::Home => page::home::view(),
            Page::Movies(model) => page::movies::view(model).map_msg(Msg::MoviesMsg),
            Page::Settings(model) => page::settings::view(model).map_msg(Msg::SettingsMsg),
            Page::NotFound => page::not_found::view(),
        }
    ]
}

// ----- view_navbar ------

fn view_navbar(menu_visible: bool, base_url: &Url, user: Option<&User>, page: &Page) -> Node<Msg> {
    nav![
        C!["navbar"],
        s()
            .bg_color("white")
            .min_h(rem(3.25))
            .position(CssPosition::Relative),
        s()
            .only_and_above(Breakpoint::Desktop)
            .align_items(CssAlignItems::Stretch)
            .display(CssDisplay::Flex),
        attrs!{
            At::from("role") => "navigation",
            At::AriaLabel => "main navigation",
        },
        view_brand_and_hamburger(menu_visible, base_url),
        view_navbar_menu(menu_visible, base_url, user, page),
    ]
}

fn view_brand_and_hamburger(menu_visible: bool, base_url: &Url) -> Node<Msg> {
    div![
        C!["navbar-brand"],
        s()
            .align_items(CssAlignItems::Stretch)
            .display(CssDisplay::Flex)
            .flex_shrink("0")
            .min_h(rem(3.25)),
        // ------ Logo ------
        a![
            C!["navbar-item", /*@TODO: Remove: "has-text-weight-bold-x", "is-size-3-x"*/],
            s()
                .display(CssDisplay::Block)
                .cursor(CssCursor::Pointer)
                .font_weight(CssFontWeight::Bold)
                .font_size(2)
                .flex_grow("0")
                .flex_shrink("0")
                .color("#4a4a4a")
                .line_height("1.5")
                .py(rem(0.5))
                .px(rem(0.75))
                .position(CssPosition::Relative),
            s()
                .only_and_above(Breakpoint::Desktop)
                .align_items(CssAlignItems::Center)
                .display(CssDisplay::Flex),
            attrs!{At::Href => Urls::new(base_url).home()},
            "SM"
        ],
        // ------ Hamburger ------
        a![
            C!["navbar-burger", /*@TODO: Remove: "burger", IF!(menu_visible => "is-active")*/],
            s()
                .color("#4a4a4a")
                .cursor(CssCursor::Pointer)
                .display(CssDisplay::Block)
                .h(rem(3.25))
                .position(CssPosition::Relative)
                .w(rem(3.25))
                .ml(CssMarginLeft::Auto),
            s()
                .only_and_above(Breakpoint::Desktop)
                .display(CssDisplay::None),
            s()
                .hover()
                .bg_color(rgba(0, 0, 0, 0.05)),
            attrs!{
                At::from("role") => "button",
                At::AriaLabel => "menu",
                At::AriaExpanded => menu_visible,
            },
            ev(Ev::Click, |event| {
                event.stop_propagation();
                Msg::ToggleMenu
            }),
            (0..3).map(|index| view_hamburger_line(index, menu_visible)),
        ]
    ]
}

fn view_hamburger_line(index: u8, menu_visible: bool) -> Node<Msg> {
    let top = match index {
        0 => "calc(50% - 6px)",
        1 => "calc(50% - 1px)",
        2 => "calc(50% + 4px)",
        _ => panic!("hamburger has only 3 lines!")
    };

    span![
        s()
            .top(top)
            .bg_color("currentColor")
            .display(CssDisplay::Block)
            .h(px(1))
            .left("calc(50% - 8px)")
            .position(CssPosition::Absolute)
            .transform_origin("center")
            .transition_duration("86ms")
            .transition_property("background-color,opacity,transform")
            .transition_timing_function("ease-out")
            .w(px(16)),
        // @TODO if/else should be replaced with IF! once possible
        if menu_visible {
            match index {
                0 => s().transform("translateY(5px) rotate(45deg)"),
                1 => s().opacity("0"),
                2 => s().transform("translateY(-5px) rotate(-45deg)"),
                _ => panic!("hamburger has only 3 lines!")
            }
        } else {
            s()
        },
        attrs!{At::AriaHidden => "true"}
    ]
}

fn view_navbar_menu(menu_visible: bool, base_url: &Url, user: Option<&User>, page: &Page) -> Node<Msg> {
    div![
        C!["navbar-menu", /*@TODO: Remove: IF!(menu_visible => "is-active")*/],
        s()
            .display(CssDisplay::None),
        s()
            .only_and_above(Breakpoint::Desktop)
            .flex_grow("1")
            .flex_shrink("0")
            .align_items(CssAlignItems::Stretch)
            .display(CssDisplay::Flex),
        // @TODO if/else should be replaced with IF! once possible
        if menu_visible {
            s()
                .only_and_below(Breakpoint::Tablet)
                .display(CssDisplay::Block)
        } else {
            s()
        },
        s()
            .only_and_below(Breakpoint::Tablet)
            .bg_color("white")
            .box_shadow("0 8px 16px rgba(10,10,10,.1)")
            .py(rem(0.5))
            .px("0"),
        view_navbar_menu_start(base_url, page),
        view_navbar_menu_end(base_url, user),
    ]
}

fn view_navbar_menu_start(base_url: &Url, page: &Page) -> Node<Msg> {
    div![
        C!["navbar-start"],
        s()
            .only_and_above(Breakpoint::Desktop)
            .justify_content(CssJustifyContent::FlexStart)
            .mr(CssMarginRight::Auto)
            .align_items(CssAlignItems::Stretch)
            .display(CssDisplay::Flex),
        {
            let is_active = matches!(page, Page::Movies(_));
            a![
                C!["navbar-item", /*@TODO: Remove: "is-tab", IF!(is_active => "is-active"),*/],
                s()
                    .display(CssDisplay::Block)
                    .border_bottom("1px solid transparent")
                    .min_h(rem(3.25))
                    .pb("calc(.5rem - 1px)")
                    .cursor(CssCursor::Pointer)
                    .flex_grow("0")
                    .flex_shrink("0")
                    .color("#4a4a4a")
                    .line_height("1.5")
                    .pt(rem(0.5))
                    .px(rem(0.75))
                    .position(CssPosition::Relative),
                s()
                    .focus()
                    .hover()
                    .bg_color("transparent")
                    .border_bottom_color("#3273dc")
                    .color("#3273dc"),
                if is_active {
                    s()
                        .bg_color("transparent")
                        .border_bottom_color("#3273dc")
                        .color("#3273dc")
                        .border_bottom_width(px(3))
                        .pb("calc(.5rem - 3px)")
                } else {
                    s()
                },
                s()
                    .only_and_above(Breakpoint::Desktop)
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex),
                attrs!{At::Href => Urls::new(base_url).movies()},
                "Movies",
            ]
        }
    ]
}

fn view_navbar_menu_end(base_url: &Url, user: Option<&User>) -> Node<Msg> {
     div![
        C!["navbar-end"],
        s()
            .only_and_above(Breakpoint::Desktop)
            .justify_content(CssJustifyContent::FlexEnd)
            .ml(CssMarginLeft::Auto)
            .align_items(CssAlignItems::Stretch)
            .display(CssDisplay::Flex),
        div![
            C!["navbar-item"],
            s()
                .display(CssDisplay::Block)
                .cursor(CssCursor::Pointer)
                .flex_grow("0")
                .flex_shrink("0")
                .color("#4a4a4a")
                .line_height("1.5")
                .py(rem(0.5))
                .px(rem(0.75))
                .position(CssPosition::Relative),
            s()
                .only_and_above(Breakpoint::Desktop)
                .align_items(CssAlignItems::Center)
                .display(CssDisplay::Flex),
            div![
                C!["buttons"],
                s()
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex_wrap(CssFlexWrap::Wrap)
                    .justify_content(CssJustifyContent::FlexStart),
                s()
                    .last_child()
                    .mb(rem(-0.5)),
                s()
                    // @TODO replace with `.not().last_child()` or something similar once possible
                    // @TODO `not` string argument with `:` doesn't work?
                    .not(":last-child")
                    .mr(rem(0.5)),
                if let Some(user) = user {
                    view_buttons_for_logged_in_user(base_url, user)
                } else {
                    view_buttons_for_anonymous_user()
                }
            ]
        ]
    ]
}

fn view_buttons_for_logged_in_user(base_url: &Url, user: &User) -> Vec<Node<Msg>> {
    vec![
        a![
            C!["button", /*@TODO: Remove: "is-primary"*/],
            s_primary_button(),
            attrs![
                At::Href => Urls::new(base_url).settings(),
            ],
            strong![&user.username],
        ],
        a![
            C!["button", /*@TODO: Remove: "is-light"*/],
            s_light_button(),
            attrs![
                // @TODO: Write the correct href.
                At::Href => "/"
            ],
            "Log out",
        ]
    ]
}

fn view_buttons_for_anonymous_user() -> Vec<Node<Msg>> {
    vec![
        a![
            C!["button", /*@TODO: Remove: "is-primary"*/],
            s_primary_button(),
            attrs![
                // @TODO: Write the correct href.
                At::Href => "/"
            ],
            strong!["Sign up"],
        ],
        a![
            C!["button", /*@TODO: Remove: "is-light"*/],
            s_light_button(),
            attrs![
                // @TODO: Write the correct href.
                At::Href => "/"
            ],
            "Log in",
        ]
    ]
}

// ------ Styles -----

fn s_primary_button() -> Vec<Style> {
    vec![
        s_button(),
        s()
            .bg_color("#00d1b2")
            .border_color("transparent")
            .color("white"),
        s()
            .hover()
            .bg_color("#00c4a7")
            .border_color("transparent")
            .color("white")
    ]
}

fn s_light_button() -> Vec<Style> {
    vec![
        s_button(),
        s()
            .bg_color("#f5f5f5")
            .border_color("transparent")
            .color(rgba(0, 0, 0, 0.7)),
        s()
            .hover()
            .bg_color("#eee")
            .border_color("transparent")
            .color(rgba(0, 0, 0, 0.7)),
    ]
}

fn s_button() -> Style {
    s()
        .mb(rem(0.5))
        .border_width(px(1))
        .cursor(CssCursor::Pointer)
        .justify_content(CssJustifyContent::Center)
        .pb("calc(.5em - 1px)")
        .px(em(1))
        .pt("calc(.5em - 1px)")
        .text_align(CssTextAlign::Center)
        .white_space(CssWhiteSpace::NoWrap)
        .raw("-moz-appearance: none;")
        .raw("-webkit-appearance: none;")
        .border("1px solid transparent")
        .border_radius(px(4))
        .box_shadow(CssBoxShadow::None)
        .display(CssDisplay::InlineFlex)
        .font_size(rem(1))
        .h(em(2.5))
        .line_height("1.5")
        .position(CssPosition::Relative)
        .user_select("none")
        .vertical_align(CssVerticalAlign::Top)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
