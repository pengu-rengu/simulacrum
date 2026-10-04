use serde::{Serialize, Deserialize};

use crate::state::agent::Agent;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Universe {
    Nodeworld, EscapeRoom
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
pub enum NodeworldAction {
    MoveTo(String),
    Harvest { poi_idx: usize, tool_idxs: Vec<Option<usize>> },
    Inspect { poi_idx: usize },
    Craft(usize),
    ExitMenu,
    Engage { poi_idx: usize }
}

#[derive(Deserialize)]
pub struct NodeworldInput {
    pub action: Option<NodeworldAction>,
    pub send_message: Option<String>
}

#[derive(Deserialize)]
pub enum Input {
    NodeworldInput(NodeworldInput)
}