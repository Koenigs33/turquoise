use yew::{function_component, html, AttrValue, Callback, Component, Context, Html, Properties};

pub struct Tile {}

#[derive(PartialEq, Clone, Debug)]
pub enum TileColor {
    Red,
    Yellow,
    Green,
    Purple,
    Blue,
}

impl TileColor {
    pub fn value(&self) -> &str {
        match self {
            TileColor::Red => "#fa5252",
            TileColor::Blue => "#15aabf",
            TileColor::Yellow => "#fab005",
            TileColor::Green => "#82c91e",
            TileColor::Purple => "#7950f2",
        }
    }
}

impl Default for TileColor {
    fn default() -> Self {
        Self::Blue
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct TileState {
    pub id: u8,
    pub color: TileColor,
    pub position: TilePosition,
    pub selected: bool,
}

impl TileState {
    pub fn new(id: u8, color: TileColor) -> Self {
        Self {
            id,
            color,
            position: TilePosition::Reserve,
            selected: false,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum TilePosition {
    Reserve,
    Pot(u8),
    CommonPot,
    CurrentBoard(AttrValue),
    PlayerBoard(AttrValue),
    Bin,
}

#[derive(Properties, PartialEq)]
pub struct TileProps {
    #[prop_or_default]
    pub color: TileColor,
    pub click_handler: Callback<(u8, TileColor)>,
    pub id: u8,
    pub selected: bool,
}

pub enum TileMsg {
    Clicked,
}

impl Component for Tile {
    type Message = TileMsg;
    type Properties = TileProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let TileProps {
            color,
            click_handler,
            id,
            selected,
        } = ctx.props();

        match msg {
            TileMsg::Clicked => {
                click_handler.emit((*id, (*color).clone()));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| TileMsg::Clicked);
        let TileProps {
            color,
            click_handler: _,
            id: _,
            selected,
        } = ctx.props();
        html! {
            <div onclick={onclick}>
                <svg version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 80 80" width="80" height="80">
                    <g stroke-linecap="round" transform="translate(10 10) rotate(0 30 30)">
                        <path
                            d="M-1.35 -1.68 L61.7 0.41 L59.46 59.49 L-0.95 60.85"
                            stroke="none"
                            stroke-width="0"
                            fill={color.value().to_owned()}
                        />
                        if *selected {
                            <BoldTileRect/>
                        } else {
                            <TileRect/>
                        }
                    </g>
                </svg>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
struct TileFillProps {
    pub color: TileColor,
}

#[function_component]
fn TileSolidFill(props: &TileFillProps) -> Html {
    let TileFillProps { color } = props;
    html! {
        <path
        d="M-1.35 -1.68 L61.7 0.41 L59.46 59.49 L-0.95 60.85"
        stroke="none"
        stroke-width="0"
        fill={color.value().to_owned()}/>
    }
}

#[function_component]
fn TileHatchFill(props: &TileFillProps) -> Html {
    let TileFillProps { color } = props;
    html! {
        <path
      d="M0 0 C0 0, 0 0, 0 0 M0 0 C0 0, 0 0, 0 0 M0.43 7.1 C1.19 4.95, 3.86 2.38, 5.55 0.93 M-0.22 6.59 C1.71 4.75, 2.85 3.07, 5.07 0.53 M-0.45 13.62 C1.94 8.08, 5.81 7.41, 10.46 1.48 M-0.09 12.21 C2.86 10.06, 4.51 7.68, 10.52 0.51 M1.46 20.02 C3.08 15.78, 6.88 10.63, 13.95 -0.35 M0.68 19.31 C4.69 12.32, 10.26 6.72, 15.95 0.46 M-0.54 25.49 C5.77 19.42, 11.1 10.55, 21.34 1.85 M-0.2 23.35 C4.89 16.72, 12.17 10.9, 21.49 0.87 M0.02 31.04 C8.1 20.72, 20.01 8.95, 27.3 1.85 M0.7 29.99 C8 20.29, 17.19 11.77, 26.8 0.59 M-1.85 37.18 C7.59 28.19, 19.68 16.84, 33.19 -0.32 M0.47 36.35 C10.34 24.04, 21.58 13.08, 32.51 -0.1 M-1.58 40.57 C7.34 30.72, 19.56 20.98, 38.81 -0.08 M-0.21 42.9 C11 31.22, 20.5 19.17, 35.98 1.19 M-1.99 48.2 C8.1 39.2, 20.52 28.07, 42.77 -0.34 M-0.04 48.2 C10.58 36.1, 23.3 21.99, 42.45 0.48 M-1.63 54.05 C20.52 35.43, 38.82 12.08, 46.25 -1.54 M-0.59 55.19 C19.74 32.01, 38.74 12.14, 46.79 -0.02 M1.69 60.29 C20.33 37.68, 37.19 16.89, 53.32 -1.79 M0.12 61.02 C14.29 44.53, 27.67 29.01, 52.72 -0.43 M5.2 58.97 C18.1 47.75, 29.85 33.35, 58.79 1.18 M4.8 60.23 C16.35 46.93, 29.36 35.22, 58.54 -0.63 M12.43 59.73 C29.88 39.2, 49.22 15.1, 62.5 0.1 M12.04 61.13 C21.91 48.97, 31.01 36.9, 62.34 1.55 M15.44 58.97 C30.76 41.36, 46.7 26.63, 60.45 8.43 M15.49 60.57 C29.53 44.83, 42.92 29.81, 63.06 7.04 M23.85 59.75 C29.26 48.05, 41.44 39.11, 61.77 15.47 M22.79 60.39 C34.16 45.59, 46.65 31.81, 62.28 14.32 M26.73 59.8 C36.95 51.6, 45.27 40.71, 63.85 21.08 M26.33 61.05 C36.55 49.11, 43.96 40.29, 61.39 18.77 M30.81 61.37 C36.92 53.15, 43.53 47.43, 62.12 27.19 M33.39 60.07 C41.57 50.35, 52.19 38.19, 62.14 25.63 M38.39 59.28 C45.28 49.96, 54.17 44.63, 63.67 32.36 M36.94 59.65 C42.2 54.78, 48.95 47.26, 62.28 33.33 M41.58 58.72 C52.3 51.81, 59.36 41.06, 60.38 40.06 M43.84 59.12 C48.52 53.34, 54.37 47.39, 61.53 37.22 M48.15 59.31 C52.86 54.47, 56.4 51.23, 60.01 44.82 M47.52 60.56 C50.76 56.55, 53.07 54.55, 62.57 44.43 M51.77 61.45 C56.81 56.75, 61.08 54.34, 61.98 49.33 M53.46 60.22 C55.3 57.92, 56.98 55.41, 61.84 50.1 M58.64 60.27 C59.68 59.25, 61.77 57.83, 61.67 56.57 M59.05 60.59 C59.55 59.49, 60.27 58.69, 62.1 56.4 M-0.27 59.76 C-0.27 59.76, -0.27 59.76, -0.27 59.76 M-0.27 59.76 C-0.27 59.76, -0.27 59.76, -0.27 59.76 M5.9 60.35 C4.39 58.73, 0.84 56.22, -0.59 54.26 M6.36 60.31 C4.67 58.67, 3.56 57.54, -0.13 54.9 M12.06 60.81 C7.98 56.02, 4.95 53.62, -1.45 47.82 M12.83 60.93 C9.66 58.13, 5.47 54.13, -0.56 49.96 M19.43 60.85 C12.99 57.46, 9.56 49.29, -0.94 42.56 M18.16 60.66 C12.37 53.51, 4.33 48.95, 0.45 44.81 M24.22 62.02 C16.05 53.67, 9.78 47.85, -1.28 38.08 M24.48 60.21 C16.81 53.85, 8.29 45.44, -1.11 38.74 M30.35 58 C21.13 51.45, 13.05 43.14, 0.85 32.82 M29.75 59.67 C19.89 50.79, 10.09 40.83, 0.63 33.79 M36.71 59.83 C24.77 46.86, 8.05 35.27, 0.11 29.7 M36.81 60.11 C27.23 52.12, 17.3 42.39, -0.03 29.13 M42.82 61.32 C28.88 48.96, 13.59 34.29, 1.62 21.85 M42.17 60.64 C30.06 50.21, 17.72 38.79, 0.84 23.75 M48.17 60.99 C35.21 45.48, 19.18 32.95, 2.29 16.48 M49.19 58.93 C35.48 48.05, 23.08 38.32, 1.03 17.93 M53.51 61.97 C35.41 44.47, 19.44 28.97, -1.41 10.28 M54.37 60.67 C43.09 49.59, 31.54 39.89, -0.13 13.16 M60.7 59.29 C47.56 48.54, 35.31 36.41, -0.31 8.52 M60.98 59.56 C48.12 48.53, 33.89 36.14, 0.27 6.96 M58.94 53.26 C45.44 39.46, 30.06 27.73, -1.58 -0.15 M60.32 54.12 C39.16 34.15, 17.72 15.43, -0.42 2.15 M61.75 51.09 C43.51 37.31, 27.43 22.94, 1.93 -3.33 M60.51 50.08 C39.14 29.9, 18.33 13.69, 1.19 -1.08 M61.59 43.17 C39.84 25.24, 20.74 9.15, 7.45 -1.39 M60.09 44.12 C44.22 29.1, 27.52 15.63, 8.16 -1.2 M62.32 36.84 C50.27 30.95, 39.98 19.5, 14.57 -3.57 M60.31 38.3 C50.34 29.18, 40.33 21.64, 14.4 -1.74 M60.13 33.05 C45.08 22.54, 33.06 12.06, 20.63 -3.46 M61.06 32.28 C50.43 23.95, 38.89 14.02, 20 -2.27 M58.74 27.59 C53.95 19.98, 45.49 14.44, 25.96 -1.66 M59.89 27.19 C50.32 19.23, 41.63 11.27, 25.4 -1.43 M58.34 23.74 C51.21 15.52, 40.65 4.61, 33.03 -0.6 M59.89 21.82 C51.68 13.92, 42.45 6.37, 32.34 -1.09 M58.76 17.49 C55.69 12.94, 47.09 6.02, 40.16 -3.14 M60.78 16.46 C54.94 11.9, 47.79 5.41, 38.6 -1.49 M60.42 10.19 C52.7 4.47, 49.19 -0.88, 44.92 -2.71 M60.77 11.16 C54.25 6.3, 47.25 0.39, 45.2 -2.68 M59.86 6.06 C57.16 3.81, 54.61 2.02, 49.85 -2.89 M59.92 7.19 C58.04 5.22, 56.06 3.31, 50.68 -1.36 M60.33 0.57 C58.56 0.25, 57.3 -0.9, 56.26 -1.89 M60.2 0.95 C58.62 -0.23, 57.44 -1.57, 56.42 -2.09"
      stroke={color.value().to_owned()} stroke-width="0.5" fill="none"></path>
    }
}

#[function_component]
pub fn TileRect() -> Html {
    html! {
        <path
            d="M-0.06 -1.98 C19.45 -2.01, 38.61 0.49, 61.86 -0.11 M-0.66 -0.26 C22.94 -0.26, 43.56 -0.27, 60.23 0.3 M59 1.98 C62.04 13.72, 58.33 26.53, 60.33 58.46 M60.14 0.93 C59.59 12.8, 58.82 25.75, 60.55 60.62 M59.7 61.56 C39.01 59.52, 16.71 61.26, -1.7 58.04 M59.08 60.52 C45.43 60.75, 29.13 60.99, 0.32 59.98 M0.5 59.45 C0.23 48.05, -0.1 34.04, -1.1 -1.44 M0.78 59.44 C-0.71 39.59, -0.51 18.93, -0.43 0.7"
            stroke="#000000"
            stroke-width="1"
            fill="none"/>
    }
}

#[function_component]
pub fn BoldTileRect() -> Html {
    html! {
        <path
            d="M1.88 0.23 C16.24 -1.04, 31.05 2.17, 58.9 0.58 M-0.52 -0.64 C23.04 -0.2, 44.32 -0.27, 59.49 -0.94 M58.01 -1.9 C60.92 13.25, 60.29 25.58, 61.73 61.8 M60.24 -0.6 C60.71 21.77, 61.13 45.18, 60.08 59.91 M60.87 61.38 C48.37 61.03, 35.22 59.04, -1.49 60.35 M60.17 59 C46.54 59.53, 31.28 59.32, 0.25 60.13 M0.16 61.99 C-0.66 47.51, -1.11 32.23, -0.59 0.25 M-0.12 60.98 C0.31 40.49, 0.26 19.21, 0.41 0.7"
            stroke="#000000"
            stroke-width="4"
            fill="none">
        </path>
    }
}

pub struct TileStateUpdate {
    pub pot_id: usize,
    pub tile_id: usize,
    pub tile_color: TileColor,
}
