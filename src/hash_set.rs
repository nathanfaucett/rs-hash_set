use core::borrow::Borrow;
use core::fmt;
use core::hash::{Hash, BuildHasher};
use core::iter::{Chain, FromIterator, FusedIterator};
use core::ops::{BitOr, BitAnd, BitXor, Sub};

use hash_map::{self, Recover, HashMap, Keys, RandomState};
use collection_traits::*;


#[derive(Clone)]
pub struct HashSet<T, S = RandomState> {
    hash_map: HashMap<T, (), S>,
}

impl<T: Hash + Eq> HashSet<T, RandomState> {
    #[inline]
    pub fn new() -> HashSet<T, RandomState> {
        HashSet { hash_map: HashMap::new() }
    }
    #[inline]
    pub fn with_capacity(capacity: usize) -> HashSet<T, RandomState> {
        HashSet { hash_map: HashMap::with_capacity(capacity) }
    }
}

impl<T, S> HashSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    pub fn with_hasher(hasher: S) -> HashSet<T, S> {
        HashSet {
            hash_map: HashMap::with_hasher(hasher),
        }
    }
    #[inline(always)]
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> HashSet<T, S> {
        HashSet {
            hash_map: HashMap::with_capacity_and_hasher(capacity, hasher),
        }
    }
    #[inline(always)]
    pub fn hasher(&self) -> &S {
        self.hash_map.hasher()
    }
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.hash_map.capacity()
    }
    #[inline(always)]
    pub fn reserve(&mut self, additional: usize) {
        self.hash_map.reserve(additional)
    }
    #[inline(always)]
    pub fn shrink_to_fit(&mut self) {
        self.hash_map.shrink_to_fit()
    }
    #[inline(always)]
    pub fn difference<'a>(&'a self, other: &'a HashSet<T, S>) -> Difference<'a, T, S> {
        Difference {
            iter: self.iter(),
            other: other,
        }
    }
    #[inline(always)]
    pub fn symmetric_difference<'a>(
        &'a self,
        other: &'a HashSet<T, S>)
    -> SymmetricDifference<'a, T, S> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }
    #[inline(always)]
    pub fn intersection<'a>(&'a self, other: &'a HashSet<T, S>) -> Intersection<'a, T, S> {
        Intersection {
            iter: self.iter(),
            other: other,
        }
    }
    #[inline(always)]
    pub fn union<'a>(&'a self, other: &'a HashSet<T, S>) -> Union<'a, T, S> {
        Union {
            iter: self.iter().chain(other.difference(self)),
        }
    }
    #[inline(always)]
    pub fn drain(&mut self) -> Drain<T> {
        Drain {
            iter: self.hash_map.drain(),
        }
    }
    #[inline(always)]
    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
        where T: Borrow<Q>,
              Q: Hash + Eq
    {
        self.hash_map.contains_key(value)
    }
    #[inline(always)]
    pub fn get<Q: ?Sized>(&self, value: &Q) -> Option<&T>
        where T: Borrow<Q>,
              Q: Hash + Eq
    {
        Recover::get(&self.hash_map, value)
    }
    #[inline(always)]
    pub fn is_disjoint(&self, other: &HashSet<T, S>) -> bool {
        self.iter().all(|v| !other.contains(v))
    }
    #[inline(always)]
    pub fn is_subset(&self, other: &HashSet<T, S>) -> bool {
        self.iter().all(|v| other.contains(v))
    }
    #[inline(always)]
    pub fn is_superset(&self, other: &HashSet<T, S>) -> bool {
        other.is_subset(self)
    }
    #[inline(always)]
    pub fn insert(&mut self, value: T) -> bool {
        self.hash_map.insert(value, ()).is_none()
    }
    #[inline(always)]
    pub fn replace(&mut self, value: T) -> Option<T> {
        Recover::replace(&mut self.hash_map, value)
    }
    #[inline(always)]
    pub fn take<Q: ?Sized>(&mut self, value: &Q) -> Option<T>
        where T: Borrow<Q>,
              Q: Hash + Eq
    {
        Recover::take(&mut self.hash_map, value)
    }
}

impl<T, S> Collection for HashSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    fn len(&self) -> usize {
        self.hash_map.len()
    }
}

impl<T, S> CollectionMut for HashSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    fn clear(&mut self) {
        self.hash_map.clear()
    }
}

impl<'a, T, Q: ?Sized, S> RemoveMut<&'a Q> for HashSet<T, S>
    where T: Eq + Hash + Borrow<Q>,
          Q: Eq + Hash,
          S: BuildHasher
{
    type Output = bool;

    #[inline(always)]
    fn remove(&mut self, value: &Q) -> Self::Output {
        self.hash_map.remove(value).is_some()
    }
}

impl<'a, T, S> Iterable<'a, &'a T> for HashSet<T, S>
    where T: 'a + Eq + Hash,
          S: 'a + BuildHasher,
{
    type Iter = Iter<'a, T>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        Iter {
            iter: self.hash_map.keys()
        }
    }
}

impl<T, S> PartialEq for HashSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    #[inline]
    fn eq(&self, other: &HashSet<T, S>) -> bool {
        if self.len() != other.len() {
            false
        } else {
            self.iter().all(|key| other.contains(key))
        }
    }
}

impl<T, S> Eq for HashSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher {}

impl<T, S> fmt::Debug for HashSet<T, S>
    where T: Eq + Hash + fmt::Debug,
          S: BuildHasher
{
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T, S> FromIterator<T> for HashSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher + Default
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> HashSet<T, S> {
        let mut set = HashSet::with_hasher(Default::default());
        set.extend(iter);
        set
    }
}

impl<T, S> Extend<T> for HashSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    #[inline(always)]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.hash_map.extend(iter.into_iter().map(|k| (k, ())));
    }
}

impl<'a, T, S> Extend<&'a T> for HashSet<T, S>
    where T: 'a + Eq + Hash + Copy,
          S: BuildHasher
{
    #[inline(always)]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T, S> Default for HashSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher + Default
{
    #[inline(always)]
    fn default() -> HashSet<T, S> {
        HashSet {
            hash_map: HashMap::default(),
        }
    }
}

impl<'a, 'b, T, S> BitOr<&'b HashSet<T, S>> for &'a HashSet<T, S>
    where T: Eq + Hash + Clone,
          S: BuildHasher + Default
{
    type Output = HashSet<T, S>;

    #[inline(always)]
    fn bitor(self, rhs: &HashSet<T, S>) -> HashSet<T, S> {
        self.union(rhs).cloned().collect()
    }
}

impl<'a, 'b, T, S> BitAnd<&'b HashSet<T, S>> for &'a HashSet<T, S>
    where T: Eq + Hash + Clone,
          S: BuildHasher + Default
{
    type Output = HashSet<T, S>;

    #[inline(always)]
    fn bitand(self, rhs: &HashSet<T, S>) -> HashSet<T, S> {
        self.intersection(rhs).cloned().collect()
    }
}

impl<'a, 'b, T, S> BitXor<&'b HashSet<T, S>> for &'a HashSet<T, S>
    where T: Eq + Hash + Clone,
          S: BuildHasher + Default
{
    type Output = HashSet<T, S>;

    #[inline(always)]
    fn bitxor(self, rhs: &HashSet<T, S>) -> HashSet<T, S> {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<'a, 'b, T, S> Sub<&'b HashSet<T, S>> for &'a HashSet<T, S>
    where T: Eq + Hash + Clone,
          S: BuildHasher + Default
{
    type Output = HashSet<T, S>;

    #[inline(always)]
    fn sub(self, rhs: &HashSet<T, S>) -> HashSet<T, S> {
        self.difference(rhs).cloned().collect()
    }
}

pub struct Iter<'a, K: 'a> {
    iter: Keys<'a, K, ()>,
}

pub struct IntoIter<K> {
    iter: hash_map::IntoIter<K, ()>,
}

pub struct Drain<'a, K: 'a> {
    iter: hash_map::Drain<'a, K, ()>,
}

pub struct Intersection<'a, T: 'a, S: 'a> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S>,
}

pub struct Difference<'a, T: 'a, S: 'a> {
    iter: Iter<'a, T>,
    other: &'a HashSet<T, S>,
}

pub struct SymmetricDifference<'a, T: 'a, S: 'a> {
    iter: Chain<Difference<'a, T, S>, Difference<'a, T, S>>,
}

pub struct Union<'a, T: 'a, S: 'a> {
    iter: Chain<Iter<'a, T>, Difference<'a, T, S>>,
}

impl<'a, T, S> IntoIterator for &'a HashSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline(always)]
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T, S> IntoIterator for HashSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline(always)]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            iter: self.hash_map.into_iter(),
        }
    }
}

impl<'a, K> Clone for Iter<'a, K> {
    #[inline(always)]
    fn clone(&self) -> Iter<'a, K> {
        Iter {
            iter: self.iter.clone(),
        }
    }
}
impl<'a, K> Iterator for Iter<'a, K> {
    type Item = &'a K;

    #[inline(always)]
    fn next(&mut self) -> Option<&'a K> {
        self.iter.next()
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'a, K> ExactSizeIterator for Iter<'a, K> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K> FusedIterator for Iter<'a, K> {}

impl<K> Iterator for IntoIter<K> {
    type Item = K;

    #[inline(always)]
    fn next(&mut self) -> Option<K> {
        self.iter.next().map(|(k, _)| k)
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<K> ExactSizeIterator for IntoIter<K> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.iter.len()
    }
}
impl<K> FusedIterator for IntoIter<K> {}

impl<'a, K> Iterator for Drain<'a, K> {
    type Item = K;

    #[inline(always)]
    fn next(&mut self) -> Option<K> {
        self.iter.next().map(|(k, _)| k)
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<'a, K> ExactSizeIterator for Drain<'a, K> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K> FusedIterator for Drain<'a, K> {}

impl<'a, T, S> Clone for Intersection<'a, T, S> {
    #[inline(always)]
    fn clone(&self) -> Intersection<'a, T, S> {
        Intersection {
            iter: self.iter.clone(),
            ..*self
        }
    }
}

impl<'a, T, S> Iterator for Intersection<'a, T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(elt) => {
                    if self.other.contains(elt) {
                        return Some(elt);
                    }
                }
            }
        }
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }
}

impl<'a, T, S> FusedIterator for Intersection<'a, T, S>
    where T: Eq + Hash,
          S: BuildHasher {}

impl<'a, T, S> Clone for Difference<'a, T, S> {
    #[inline(always)]
    fn clone(&self) -> Difference<'a, T, S> {
        Difference { iter: self.iter.clone(), ..*self }
    }
}

impl<'a, T, S> Iterator for Difference<'a, T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(elt) => {
                    if !self.other.contains(elt) {
                        return Some(elt);
                    }
                }
            }
        }
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }
}

impl<'a, T, S> FusedIterator for Difference<'a, T, S>
    where T: Eq + Hash,
          S: BuildHasher {}

impl<'a, T, S> Clone for SymmetricDifference<'a, T, S> {
    #[inline(always)]
    fn clone(&self) -> SymmetricDifference<'a, T, S> {
        SymmetricDifference { iter: self.iter.clone() }
    }
}

impl<'a, T, S> Iterator for SymmetricDifference<'a, T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T, S> FusedIterator for SymmetricDifference<'a, T, S>
    where T: Eq + Hash,
          S: BuildHasher {}

impl<'a, T, S> Clone for Union<'a, T, S> {
    #[inline(always)]
    fn clone(&self) -> Union<'a, T, S> {
        Union {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, T, S> FusedIterator for Union<'a, T, S>
    where T: Eq + Hash,
          S: BuildHasher {}

impl<'a, T, S> Iterator for Union<'a, T, S>
    where T: Eq + Hash,
          S: BuildHasher
{
    type Item = &'a T;

    #[inline(always)]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }
    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[allow(dead_code)]
fn assert_covariance() {
    #[inline(always)]
    fn set<'new>(v: HashSet<&'static str>) -> HashSet<&'new str> {
        v
    }
    #[inline(always)]
    fn iter<'a, 'new>(v: Iter<'a, &'static str>) -> Iter<'a, &'new str> {
        v
    }
    #[inline(always)]
    fn into_iter<'new>(v: IntoIter<&'static str>) -> IntoIter<&'new str> {
        v
    }
    #[inline(always)]
    fn difference<'a, 'new>(
        v: Difference<'a, &'static str, RandomState>)
    -> Difference<'a, &'new str, RandomState> {
        v
    }
    #[inline(always)]
    fn symmetric_difference<'a, 'new>(
        v: SymmetricDifference<'a, &'static str, RandomState>)
    -> SymmetricDifference<'a, &'new str, RandomState> {
        v
    }
    #[inline(always)]
    fn intersection<'a, 'new>(
        v: Intersection<'a, &'static str, RandomState>)
    -> Intersection<'a, &'new str, RandomState> {
        v
    }
    #[inline(always)]
    fn union<'a, 'new>(
        v: Union<'a, &'static str, RandomState>)
    -> Union<'a, &'new str, RandomState> {
        v
    }
    #[inline(always)]
    fn drain<'new>(d: Drain<'static, &'static str>) -> Drain<'new, &'new str> {
        d
    }
}
