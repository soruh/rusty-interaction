use serde::{Deserialize, Serialize};

use serde_with::*;

use super::Snowflake;
use serde_repr::*;

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
struct ApplicationCommand {
    #[serde_as(as = "DisplayFromStr")]
    id: Snowflake,
    #[serde_as(as = "DisplayFromStr")]
    application_id: Snowflake,
    name: String,
    description: String,
    options: Vec<ApplicationCommandOption>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct ApplicationCommandOption {
    r#type: i8,
    name: String,
    description: String,
    required: bool,
    choices: Vec<ApplicationCommandOptionChoice>,
    options: Vec<ApplicationCommandOption>,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct ApplicationCommandOptionChoice {
    name: String,
    // This can be int
    value: String,
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug)]
/// Representing a slash command
pub struct ApplicationCommandInteractionData {
    #[serde_as(as = "DisplayFromStr")]
    /// The unique id of the command
    pub id: Snowflake,
    /// The name of the command
    pub name: String,
    /// An array of [`ApplicationCommandInteractionDataOption`]
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
/// Representing a bunch of options for slash commands
pub struct ApplicationCommandInteractionDataOption {
    /// Name of the option
    pub name: String,
    /// Value of the option
    pub value: String,
    /// More options
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}
