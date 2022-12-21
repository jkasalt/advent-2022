use std::fmt;
use std::ops;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Matrix<T> {
    pub vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    pub fn new<I>(items: I, width: usize, height: usize) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            vec: items.into_iter().collect(),
            width,
            height,
        }
    }
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0
            || x > (self.width - 1).try_into().unwrap()
            || y < 0
            || y > (self.height - 1).try_into().unwrap()
        {
            return None;
        }
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        self.vec.get(x + y * self.width)
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if x < 0
            || x > (self.width - 1).try_into().unwrap()
            || y < 0
            || y > (self.height - 1).try_into().unwrap()
        {
            return None;
        }
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        self.vec.get_mut(x + y * self.width)
    }

    pub fn rook_neighbor_indices(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut res = Vec::new();
        if y > 0 {
            res.push((x, y - 1));
        }
        if x > 0 {
            res.push((x - 1, y));
        }
        if y < self.height - 1 {
            res.push((x, y + 1));
        }
        if x < self.width - 1 {
            res.push((x + 1, y));
        }
        res.into_iter()
    }

    pub fn neighbor_indices(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let mut xes = vec![x];
        let mut yes = vec![y];
        if y > 0 {
            yes.push(y - 1);
        }
        if y < self.height - 1 {
            yes.push(y + 1)
        }
        if x > 0 {
            xes.push(x - 1);
        }
        if x < self.width - 1 {
            xes.push(x + 1);
        }
        for yy in yes {
            for &xx in &xes {
                if xx == x && yy == y {
                    continue;
                }
                res.push((xx, yy));
            }
        }
        res
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn len(&self) -> usize {
        self.height * self.width
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn insert_row_at(&mut self, mut other: Matrix<T>, at: usize) {
        assert!(other.width() == self.width());
        let idx = at * self.width();
        self.height += other.height;
        let mut after = self.vec.split_off(idx);
        self.vec.append(&mut other.vec);
        self.vec.append(&mut after);
    }

    pub fn expand_contour(self, n: usize, with: T) -> Self
    where
        T: Clone,
    {
        let height = self.height() + n * 2;
        let width = self.width() + n * 2;

        let mut new = Matrix {
            vec: vec![with; height * width],
            height,
            width,
        };
        for x in 0..self.width {
            for y in 0..self.height {
                new[(x + n, y + n)] = self[(x, y)].clone();
            }
        }
        new
    }

    pub fn new_default(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        Matrix {
            vec: vec![T::default(); height * width],
            height,
            width,
        }
    }

    pub fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let idx_a = a.1 * self.width() + a.0;
        let idx_b = b.1 * self.width() + b.0;
        self.vec.swap(idx_a, idx_b)
    }

    pub fn index_of<F>(&self, f: F ) -> Option<(usize, usize)> where F:FnMut(&T) -> bool {
        self.vec.iter().position(f).map(|pos|(pos/self.width, pos%self.width))
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        writeln!(f)?;
        for item in &self.vec {
            write!(f, "{:?}", item)?;
            count += 1;
            if count == self.width {
                count = 0;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T> ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x > self.width - 1 || y > self.height - 1 {
            panic!(
                "Index ({}, {}) out of range for Matrix with size ({}, {})",
                y, x, self.height, self.width
            );
        }
        &self.vec[x + y * self.width]
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x > self.width - 1 || y > self.height - 1 {
            panic!(
                "Index ({}, {}) out of range for Matrix with size ({}, {})",
                y, x, self.height, self.width
            );
        }
        &mut self.vec[x + y * self.width]
    }
}

impl<T> ops::Index<(&usize, &usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (&usize, &usize)) -> &Self::Output {
        if *x > self.width - 1 || *y > self.height - 1 {
            panic!(
                "Index ({}, {}) out of range for Matrix with size ({}, {})",
                y, x, self.height, self.width
            );
        }
        &self.vec[x + y * self.width]
    }
}

impl<T> ops::IndexMut<(&usize, &usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (&usize, &usize)) -> &mut Self::Output {
        if *x > self.width - 1 || *y > self.height - 1 {
            panic!(
                "Index ({}, {}) out of range for Matrix with size ({}, {})",
                y, x, self.height, self.width
            );
        }
        &mut self.vec[x + y * self.width]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_swap() {
        let items = 0..6;
        let mut matrix = Matrix::new(items, 3, 2);
        matrix.swap((0,0), (0,1));
        assert_eq!(matrix[(0,0)], 3);
        assert_eq!(matrix[(0,1)], 0);
    }
}
