use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Agent {
    pub name: String,
    pub node_idx: usize,
    pub node_messages_inbox: Vec<Message>,
    pub error_message: Option<String>,
    pub body_grid: BodyGrid,
    pub inventory: Vec<ItemStack>,
    pub open_menu: Option<Menu>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Menu {
    Workbench
}

#[derive(Debug, Clone, PartialEq)]
pub struct Recipe {
    pub output: ItemStack,
    pub ingredients: &'static [ItemStack]
}

pub const WORKBENCH_RECIPES: &[Recipe] = &[Recipe {
    output: ItemStack { item: Item::CrudePickaxe { durability: 20 }, count: 1 },
    ingredients: &[
        ItemStack { item: Item::Stick, count: 10 },
        ItemStack { item: Item::SmoothPebble, count: 10 },
    ],
}];


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Item {
    Stick,
    Resin,
    SmoothPebble,
    CopperOre,
    CrudePickaxe { durability: usize }
}





#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ItemStack {
    pub item: Item,
    pub count: usize
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BodyCell {
    CoreCell { health: usize },
    EmptyCell
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BodyGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<BodyCell>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub sender_agent_idx: usize,
    pub content: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    pub name: String,
    pub biome: String,
    pub pois: Vec<PointOfInterest>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PoiKind {
    Thornbush,
    AmberBole,
    SmoothPebble,
    CopperOreVein,
    RuinedWorkbench
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Deposit {
    pub exposure: usize,
    pub stability: usize,
    pub reserves: usize
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PointOfInterest {
    pub kind: PoiKind,
    pub deposit: Option<Deposit>
}

/// What one swing yields, and whether it needs a pickaxe. A kind absent here cannot be harvested.
pub const POI_YIELDS: &[(PoiKind, Item, bool)] = &[
    (PoiKind::Thornbush, Item::Stick, false),
    (PoiKind::AmberBole, Item::Resin, false),
    (PoiKind::SmoothPebble, Item::SmoothPebble, false),
    (PoiKind::CopperOreVein, Item::CopperOre, true),
];

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    pub turn: usize,
    pub agent_idx: usize,
    pub agents: Vec<Agent>,
    pub nodes: Vec<Node>
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    MoveTo(String),
    Harvest { poi_idx: usize, tool_idx: Option<usize>, uses: usize },
    Inspect { poi_idx: usize },
    Craft(usize),
    Exit
}

#[derive(Deserialize)]
pub struct Input {
    pub action: Option<Action>,
    pub send_message: Option<String>
}