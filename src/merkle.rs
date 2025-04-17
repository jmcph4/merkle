use std::{convert::AsRef, fmt};

use sha2::{Digest as ShaDigest, Sha256};

pub type Bytes = Vec<u8>;

#[derive(Clone, Debug)]
pub struct Plaintext(pub Bytes);

impl AsRef<[u8]> for Plaintext {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<Bytes> for Plaintext {
    fn from(value: Bytes) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Digest(pub Bytes);

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<Bytes> for Digest {
    fn from(value: Bytes) -> Self {
        Self(value)
    }
}

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0.clone()))
    }
}

/// Given two Merkle tree nodes, concatenates their respective digests
pub fn concat_trees(left: MerkleTree, right: MerkleTree) -> Plaintext {
    concat_digests(left.root().clone(), right.root().clone())
}

/// Given two Merkle tree nodes, concatenates their respective digests
pub fn concat_digests(left: Digest, right: Digest) -> Plaintext {
    Plaintext([left.0, right.0].iter().flatten().cloned().collect())
}

/// Computes a digest
pub fn somehow_hash<T>(x: T) -> Digest
where
    T: AsRef<[u8]>,
{
    let mut hasher = Sha256::new();
    hasher.update(x);
    Digest(hasher.finalize().to_vec())
}

#[derive(Clone, Debug)]
pub enum MerkleTree {
    Internal((Box<MerkleTree>, Box<MerkleTree>)),
    Leaf(Box<Digest>),
}

impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Leaf(x) => write!(f, "{}", hex::encode(*x.clone())),
            Self::Internal((left_child, right_child)) => {
                write!(f, "({}, {})", left_child, right_child)
            }
        }
    }
}

impl MerkleTree {
    /// Constructs a new Merkle tree from an array of elements
    pub fn new<T>(elems: Vec<T>) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut levels: Vec<Vec<MerkleTree>> =
            vec![Self::compute_leaves(elems)];

        while levels[0].len() > 1 {
            levels.insert(0, Self::compute_next_level(&levels[0]));
        }

        levels[0][0].clone()
    }

    fn compute_leaves<T>(elems: Vec<T>) -> Vec<MerkleTree>
    where
        T: AsRef<[u8]>,
    {
        elems
            .iter()
            .map(somehow_hash)
            .map(|digest| MerkleTree::Leaf(Box::new(digest)))
            .collect()
    }

    fn compute_next_level(level: &[MerkleTree]) -> Vec<MerkleTree> {
        level
            .chunks(2)
            .map(|xs| {
                MerkleTree::Internal((
                    Box::new(xs[0].clone()),
                    Box::new(xs[1].clone()),
                ))
            })
            .collect()
    }

    /// Compute the root digest of the Merkle tree
    pub fn root(&self) -> Digest {
        match self {
            Self::Internal((left_child, right_child)) => somehow_hash(
                concat_trees(*left_child.clone(), *right_child.clone()),
            ),
            Self::Leaf(x) => *x.clone(),
        }
    }

    /// Calculates the height of the Merkle tree
    pub fn height(&self) -> usize {
        (match self {
            Self::Internal((left_child, _)) => left_child.height(),
            Self::Leaf(_) => 0,
        }) + 1
    }

    /// Determines whether or not a given element is a member of the underlying
    /// Merkle set
    pub fn verify(&self, leaf: Digest, proof: Vec<Digest>) -> bool {
        if proof.len() != self.height() {
            false
        } else if !self.leaves().contains(&leaf) {
            false
        } else {
            loop {
                let mut curr_digest: Digest = leaf.clone();

                for proof_digest in &proof {
                    curr_digest = somehow_hash(concat_digests(
                        curr_digest,
                        proof_digest.clone(),
                    ));
                }
            }
        }
    }

    fn leaves(&self) -> Vec<Digest> {
        match self {
            Self::Internal((left_child, right_child)) => {
                [left_child.leaves(), right_child.leaves()]
                    .iter()
                    .flatten()
                    .cloned()
                    .collect()
            }
            Self::Leaf(x) => vec![*x.clone()],
        }
    }

    /// Produces the array representation of the given Merkle tree
    ///
    /// As Merkle trees are always complete binary trees, no space is wasted.
    pub fn flatten(root: MerkleTree) -> Vec<MerkleTree> {
        let mut xs: Vec<MerkleTree> = vec![root.clone()];

        if let Self::Internal((left_child, right_child)) = root {
            xs.extend(Self::flatten(*left_child));
            xs.extend(Self::flatten(*right_child));
        }

        xs
    }
}
