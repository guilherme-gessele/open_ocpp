/// Generic used case insensitive string of 20 characters.
pub struct CiString20Type {
    cistring20: String,
}

impl CiString20Type {
    fn new(cistring20: String) -> CiString20Type {
        if cistring20.len() > 20 {
            panic!("Max length of 20 characters");
        }

        CiString20Type { cistring20 }
    }
}

impl PartialEq for CiString20Type {
    fn eq(&self, other: &Self) -> bool {
        self.cistring20.to_lowercase() == other.cistring20.to_lowercase()
    }
}

/// Generic used case insensitive string of 25 characters.
pub struct CiString25Type {
    cistring25: String,
}

impl CiString25Type {
    fn new(cistring25: String) -> CiString25Type {
        if cistring25.len() > 25 {
            panic!("Max length of 25 characters");
        }

        CiString25Type { cistring25 }
    }
}

impl PartialEq for CiString25Type {
    fn eq(&self, other: &Self) -> bool {
        self.cistring25.to_lowercase() == other.cistring25.to_lowercase()
    }
}

/// Generic used case insensitive string of 50 characters.
pub struct CiString50Type {
    cistring50: String,
}

impl CiString50Type {
    fn new(cistring50: String) -> CiString50Type {
        if cistring50.len() > 50 {
            panic!("Max length of 50 characters");
        }

        CiString50Type { cistring50 }
    }
}

impl PartialEq for CiString50Type {
    fn eq(&self, other: &Self) -> bool {
        self.cistring50.to_lowercase() == other.cistring50.to_lowercase()
    }
}

/// Generic used case insensitive string of 255 characters.
pub struct CiString255Type {
    cistring255: String,
}

impl CiString255Type {
    fn new(cistring255: String) -> CiString255Type {
        if cistring255.len() > 255 {
            panic!("Max length of 255 characters");
        }

        CiString255Type { cistring255 }
    }
}

impl PartialEq for CiString255Type {
    fn eq(&self, other: &Self) -> bool {
        self.cistring255.to_lowercase() == other.cistring255.to_lowercase()
    }
}

/// Generic used case insensitive string of 500 characters.
pub struct CiString500Type {
    cistring500: String,
}

impl CiString500Type {
    fn new(cistring500: String) -> CiString500Type {
        if cistring500.len() > 500 {
            panic!("Max length of 500 characters");
        }

        CiString500Type { cistring500 }
    }
}

impl PartialEq for CiString500Type {
    fn eq(&self, other: &Self) -> bool {
        self.cistring500.to_lowercase() == other.cistring500.to_lowercase()
    }
}
