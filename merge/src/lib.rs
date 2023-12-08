mod merge;
mod merge_error;
mod merged_cst_node;
mod ordered_merge;
mod unordered_merge;

use ordered_merge::ordered_merge;
use unordered_merge::unordered_merge;

pub use merge::merge;
pub use merge_error::MergeError;
pub use merged_cst_node::MergedCSTNode;
