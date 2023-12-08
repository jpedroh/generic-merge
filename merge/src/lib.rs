mod merge;
mod merged_cst_node;
mod ordered_merge;
mod unordered_merge;
mod merge_error;

use ordered_merge::ordered_merge;
use unordered_merge::unordered_merge;

pub use merge::merge;
pub use merged_cst_node::MergedCSTNode;
pub use merge_error::MergeError;
