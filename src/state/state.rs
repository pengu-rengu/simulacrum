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
    pub ingredients: Vec<ItemStack>
}

pub fn workbench_recipes() -> Vec<Recipe> {
    vec![Recipe {
        output: ItemStack { item: Item::CrudePickaxe, count: 1 },
        ingredients: vec![
            ItemStack { item: Item::Stick, count: 10 },
            ItemStack { item: Item::SmoothPebble, count: 10 },
        ],
    }]
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Item {
    Stick,
    Resin,
    SmoothPebble,
    CopperOre,
    CrudePickaxe
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
pub enum PointOfInterest {
    Thornbush,
    AmberBole,
    SmoothPebble,
    CopperOreVein,
    RuinedWorkbench
}

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
    Interact(usize),
    Craft(usize),
    Exit
}

#[derive(Deserialize)]
pub struct Input {
    pub action: Option<Action>,
    pub send_message: Option<String>
}