mod merge;
mod merged_cst_node;
mod ordered_merge;
mod unordered_merge;

use merged_cst_node::MergedCSTNode;
use ordered_merge::ordered_merge;
use unordered_merge::unordered_merge;

pub use merge::merge;
