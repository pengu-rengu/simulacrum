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
pub enum ToolAttribute {
    Chipping, Drilling
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Item {
    Material { name: String },
    Tool { name: String, deposit_type: DepositType, attributes: Vec<(ToolAttribute, usize)>, durability: usize }
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
    pub pois: Vec<PointOfInterest>,
    pub combat_encounters: Vec<CombatEncounter>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Enemy {
    pub name: String,
    pub body_grid: BodyGrid
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CombatEncounter {
    Pve {
        enemy_group_poi_idx: usize,
        agent_idxs: Vec<usize>,
        enemies: Vec<Enemy>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PointOfInterest {
    ResourceDeposit {
        name: String,
        type_: DepositType,
        exposed: usize,
        stability: usize,
        reserves: usize,
        yield_: Item,
    },
    Inspectable {
        name: String,
        menu: Menu
    },
    EnemyGroup {
        name: String,
        enemies: Vec<Enemy>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DepositType {
    Rock, Forage
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Menu {
    CraftingMenu { recipes: Vec<Recipe> }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Recipe {
    pub output: ItemStack,
    pub ingredients: Vec<(String, usize)>
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
    Harvest { poi_idx: usize, tool_idxs: Vec<Option<usize>> },
    Inspect { poi_idx: usize },
    Craft(usize),
    Exit,
    Engage { poi_idx: usize }
}

#[derive(Deserialize)]
pub struct Input {
    pub action: Option<Action>,
    pub send_message: Option<String>
}