pub struct Matrix {
	rows: usize,
	cols: usize,
	data: Vec<Vec<f64>>,
}

impl Matrix {
	#[allow(dead_code)]
	pub fn new( rows: usize, cols: usize, data: Vec<Vec<f64>> ) -> Matrix {
		assert!( rows > 0, "Matrix must have at least one row" );
		assert!( cols > 0, "Matrix must have at least one column" );
		assert_eq!( data.len(), rows, "Row count mismatch" );
		assert!( data.iter().all( |row| row.len() == cols ), "Column count mismatch in at least one row" );
		Matrix { rows, cols, data }
	}

	#[allow(dead_code)]
	pub fn zeros( rows: usize, cols: usize ) -> Matrix {
		assert!( rows > 0, "Matrix must have at least one row" );
		assert!( cols > 0, "Matrix must have at least one column" );
		let data = vec![vec![0.0; cols]; rows];
		Matrix { rows, cols, data }
	}

	#[allow(dead_code)]	
	pub fn shape( &self ) -> ( usize, usize ) {
		( self.rows, self.cols )
	}

	#[allow(dead_code)]	
	pub fn is_square( &self ) -> bool {
		self.rows == self.cols
	}

	#[allow(dead_code)]	
	pub fn print( &self ) {
		for row in &self.data {
			println!( "{:?}", row );
		}
	}


}


// use std::fmt;

// impl fmt::Display for Matrix {
// 	fn fmt( &self, f:)
// } 
