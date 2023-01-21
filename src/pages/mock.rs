use crate::pages::pexels::{PexlesJSON};
use crate::components::pexels_photo::{Photo, PhotoSrcSet };

pub struct Mock {

}

impl Mock {
  pub fn pexel_curated () -> PexlesJSON {
    PexlesJSON {
      next_page: "https://api.pexels.com/v1/curated/?page=2&per_page=20".to_string(),
      page: 1,
      per_page: 20,
      photos: vec![
        Photo {
             alt: "".to_string(),
             avg_color: "#A59989".to_string(),
             height: 3562,
             id: 15113967,
             liked: false,
             photographer: "stayhereforu".to_string(),
             photographer_id: 216201233,
             photographer_url: "https://www.pexels.com/@stayhereforu-216201233".to_string(),
             src: PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15113967/pexels-photo-15113967.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15113967/pexels-photo-15113967.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15113967/pexels-photo-15113967.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15113967/pexels-photo-15113967.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15113967/pexels-photo-15113967.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15113967/pexels-photo-15113967.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15113967/pexels-photo-15113967.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15113967/pexels-photo-15113967.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/woman-and-child-standing-near-wooden-house-15113967/".to_string(),
             width: 2397
        },
Photo {
             alt: "HELADOFELIZ".to_string(),
             avg_color: "#C3D3C6".to_string(),
             height: 4363,
             id: 15190699,
             liked: false,
             photographer: "Andrea Garibay".to_string(),
             photographer_id: 280714993,
             photographer_url: "https://www.pexels.com/@andreagaribay".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15190699/pexels-photo-15190699.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15190699/pexels-photo-15190699.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15190699/pexels-photo-15190699.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15190699/pexels-photo-15190699.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15190699/pexels-photo-15190699.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15190699/pexels-photo-15190699.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15190699/pexels-photo-15190699.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15190699/pexels-photo-15190699.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/heladofeliz-15190699/".to_string(),
             width: 2909
        },
Photo {
             alt: "".to_string(),
             avg_color: "#6D696A".to_string(),
             height: 9337,
             id: 15067963,
             liked: false,
             photographer: "Ricardo Oliveira".to_string(),
             photographer_id: 317251078,
             photographer_url: "https://www.pexels.com/@ricardo-oliveira-317251078".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15067963/pexels-photo-15067963.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15067963/pexels-photo-15067963.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15067963/pexels-photo-15067963.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15067963/pexels-photo-15067963.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15067963/pexels-photo-15067963.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15067963/pexels-photo-15067963.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15067963/pexels-photo-15067963.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15067963/pexels-photo-15067963.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/bottle-of-beer-and-glass-15067963/".to_string(),
             width: 6078
        },
Photo {
             alt: "seize the day".to_string(),
             avg_color: "#362E2B".to_string(),
             height: 4677,
             id: 15185101,
             liked: false,
             photographer: "Phil Desforges".to_string(),
             photographer_id: 1226422,
             photographer_url: "https://www.pexels.com/@storybyphil".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15185101/pexels-photo-15185101.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15185101/pexels-photo-15185101.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15185101/pexels-photo-15185101.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15185101/pexels-photo-15185101.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15185101/pexels-photo-15185101.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15185101/pexels-photo-15185101.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15185101/pexels-photo-15185101.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15185101/pexels-photo-15185101.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/seize-the-day-15185101/".to_string(),
             width: 3742
        },
Photo {
             alt: "".to_string(),
             avg_color: "#45443F".to_string(),
             height: 4032,
             id: 15176434,
             liked: false,
             photographer: "Eugenia Remark".to_string(),
             photographer_id: 5767088,
             photographer_url: "https://www.pexels.com/@eugenia-remark-5767088".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15176434/pexels-photo-15176434.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15176434/pexels-photo-15176434.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15176434/pexels-photo-15176434.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15176434/pexels-photo-15176434.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15176434/pexels-photo-15176434.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15176434/pexels-photo-15176434.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15176434/pexels-photo-15176434.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15176434/pexels-photo-15176434.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/fashion-man-woman-dark-15176434/".to_string(),
             width: 3024
        },
Photo {
             alt: "".to_string(),
             avg_color: "#9C8D7A".to_string(),
             height: 4032,
             id: 15176455,
             liked: false,
             photographer: "Eugenia Remark".to_string(),
             photographer_id: 5767088,
             photographer_url: "https://www.pexels.com/@eugenia-remark-5767088".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15176455/pexels-photo-15176455.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15176455/pexels-photo-15176455.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15176455/pexels-photo-15176455.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15176455/pexels-photo-15176455.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15176455/pexels-photo-15176455.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15176455/pexels-photo-15176455.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15176455/pexels-photo-15176455.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15176455/pexels-photo-15176455.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/food-wood-light-people-15176455/".to_string(),
             width: 3024
        },
Photo {
             alt: "".to_string(),
             avg_color: "#858E93".to_string(),
             height: 4032,
             id: 15176533,
             liked: false,
             photographer: "Eugenia Remark".to_string(),
             photographer_id: 5767088,
             photographer_url: "https://www.pexels.com/@eugenia-remark-5767088".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15176533/pexels-photo-15176533.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15176533/pexels-photo-15176533.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15176533/pexels-photo-15176533.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15176533/pexels-photo-15176533.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15176533/pexels-photo-15176533.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15176533/pexels-photo-15176533.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15176533/pexels-photo-15176533.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15176533/pexels-photo-15176533.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/landscape-people-relaxation-field-15176533/".to_string(),
             width: 3024
        },
Photo {
             alt: "".to_string(),
             avg_color: "#6D6960".to_string(),
             height: 6000,
             id: 15171147,
             liked: false,
             photographer: "Nati".to_string(),
             photographer_id: 87264186,
             photographer_url: "https://www.pexels.com/@nati-87264186".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15171147/pexels-photo-15171147.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15171147/pexels-photo-15171147.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15171147/pexels-photo-15171147.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15171147/pexels-photo-15171147.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15171147/pexels-photo-15171147.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15171147/pexels-photo-15171147.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15171147/pexels-photo-15171147.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15171147/pexels-photo-15171147.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/love-garden-leaf-easter-15171147/".to_string(),
             width: 4000
        },
Photo {
             alt: "".to_string(),
             avg_color: "#4A341F".to_string(),
             height: 6240,
             id: 15154492,
             liked: false,
             photographer: "Nina".to_string(),
             photographer_id: 333086571,
             photographer_url: "https://www.pexels.com/@nina-333086571".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15154492/pexels-photo-15154492.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15154492/pexels-photo-15154492.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15154492/pexels-photo-15154492.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15154492/pexels-photo-15154492.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15154492/pexels-photo-15154492.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15154492/pexels-photo-15154492.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15154492/pexels-photo-15154492.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15154492/pexels-photo-15154492.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/landscape-man-people-woman-15154492/".to_string(),
             width: 4160
        },
Photo {
             alt: "".to_string(),
             avg_color: "#7B706A".to_string(),
             height: 6240,
             id: 15045052,
             liked: false,
             photographer: "Liane  Cumming".to_string(),
             photographer_id: 224181355,
             photographer_url: "https://www.pexels.com/@liane-cumming-224181355".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15045052/pexels-photo-15045052.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15045052/pexels-photo-15045052.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15045052/pexels-photo-15045052.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15045052/pexels-photo-15045052.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15045052/pexels-photo-15045052.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15045052/pexels-photo-15045052.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15045052/pexels-photo-15045052.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15045052/pexels-photo-15045052.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/close-up-of-man-hands-holding-flower-15045052/".to_string(),
             width: 4160
        },
Photo {
             alt: "".to_string(),
             avg_color: "#64574E".to_string(),
             height: 6240,
             id: 15045086,
             liked: false,
             photographer: "Liane  Cumming".to_string(),
             photographer_id: 224181355,
             photographer_url: "https://www.pexels.com/@liane-cumming-224181355".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15045086/pexels-photo-15045086.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15045086/pexels-photo-15045086.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15045086/pexels-photo-15045086.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15045086/pexels-photo-15045086.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15045086/pexels-photo-15045086.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15045086/pexels-photo-15045086.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15045086/pexels-photo-15045086.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15045086/pexels-photo-15045086.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/woman-in-hat-posing-in-car-15045086/".to_string(),
             width: 4160
        },
Photo {
             alt: "".to_string(),
             avg_color: "#9E9694".to_string(),
             height: 6240,
             id: 15045092,
             liked: false,
             photographer: "Liane  Cumming".to_string(),
             photographer_id: 224181355,
             photographer_url: "https://www.pexels.com/@liane-cumming-224181355".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15045092/pexels-photo-15045092.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15045092/pexels-photo-15045092.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15045092/pexels-photo-15045092.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15045092/pexels-photo-15045092.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15045092/pexels-photo-15045092.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15045092/pexels-photo-15045092.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15045092/pexels-photo-15045092.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15045092/pexels-photo-15045092.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/vintage-car-near-building-15045092/".to_string(),
             width: 4160
        },
Photo {
             alt: "".to_string(),
             avg_color: "#AE9495".to_string(),
             height: 3000,
             id: 15032223,
             liked: false,
             photographer: "Mitch Lally".to_string(),
             photographer_id: 401527143,
             photographer_url: "https://www.pexels.com/@mitch-lally-401527143".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15032223/pexels-photo-15032223.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15032223/pexels-photo-15032223.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15032223/pexels-photo-15032223.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15032223/pexels-photo-15032223.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15032223/pexels-photo-15032223.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15032223/pexels-photo-15032223.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15032223/pexels-photo-15032223.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15032223/pexels-photo-15032223.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/low-angle-view-of-a-building-15032223/".to_string(),
             width: 2000
        },
Photo {
             alt: "".to_string(),
             avg_color: "#A2968A".to_string(),
             height: 5420,
             id: 15031643,
             liked: false,
             photographer: "Mitch Lally".to_string(),
             photographer_id: 401527143,
             photographer_url: "https://www.pexels.com/@mitch-lally-401527143".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15031643/pexels-photo-15031643.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15031643/pexels-photo-15031643.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15031643/pexels-photo-15031643.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15031643/pexels-photo-15031643.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15031643/pexels-photo-15031643.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15031643/pexels-photo-15031643.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15031643/pexels-photo-15031643.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15031643/pexels-photo-15031643.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/woman-in-summer-outfit-15031643/".to_string(),
             width: 3613
        },
Photo {
             alt: "".to_string(),
             avg_color: "#B1B1A9".to_string(),
             height: 6649,
             id: 15031715,
             liked: false,
             photographer: "Mitch Lally".to_string(),
             photographer_id: 401527143,
             photographer_url: "https://www.pexels.com/@mitch-lally-401527143".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15031715/pexels-photo-15031715.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15031715/pexels-photo-15031715.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15031715/pexels-photo-15031715.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15031715/pexels-photo-15031715.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15031715/pexels-photo-15031715.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15031715/pexels-photo-15031715.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15031715/pexels-photo-15031715.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15031715/pexels-photo-15031715.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/woman-standing-by-the-white-wall-15031715/".to_string(),
             width: 4433
        },
Photo {
             alt: "Free stock photo of canon rp, gor, got".to_string(),
             avg_color: "#A09B98".to_string(),
             height: 4728,
             id: 15023486,
             liked: false,
             photographer: "Carlo Obrien".to_string(),
             photographer_id: 402486657,
             photographer_url: "https://www.pexels.com/@carlo-obrien-402486657".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15023486/pexels-photo-15023486.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15023486/pexels-photo-15023486.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15023486/pexels-photo-15023486.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15023486/pexels-photo-15023486.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15023486/pexels-photo-15023486.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15023486/pexels-photo-15023486.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15023486/pexels-photo-15023486.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15023486/pexels-photo-15023486.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/rocks-in-ocean-on-sunset-15023486/".to_string(),
             width: 3782
        },
Photo {
             alt: "".to_string(),
             avg_color: "#8F9B9B".to_string(),
             height: 4033,
             id: 15113669,
             liked: false,
             photographer: "Nikita Igonkin".to_string(),
             photographer_id: 123399352,
             photographer_url: "https://www.pexels.com/@igonkin".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15113669/pexels-photo-15113669.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15113669/pexels-photo-15113669.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15113669/pexels-photo-15113669.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15113669/pexels-photo-15113669.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15113669/pexels-photo-15113669.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15113669/pexels-photo-15113669.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15113669/pexels-photo-15113669.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15113669/pexels-photo-15113669.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/a-picture-of-a-busy-roundabout-15113669/".to_string(),
             width: 2999
        },
Photo {
             alt: "".to_string(),
             avg_color: "#837665".to_string(),
             height: 6240,
             id: 15020597,
             liked: false,
             photographer: "Ozan Çulha".to_string(),
             photographer_id: 1512687,
             photographer_url: "https://www.pexels.com/@ozanculha".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15020597/pexels-photo-15020597.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15020597/pexels-photo-15020597.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15020597/pexels-photo-15020597.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15020597/pexels-photo-15020597.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15020597/pexels-photo-15020597.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15020597/pexels-photo-15020597.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15020597/pexels-photo-15020597.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15020597/pexels-photo-15020597.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/young-woman-posing-on-a-chair-15020597/".to_string(),
             width: 4160
        },
Photo {
             alt: "To Eternity".to_string(),
             avg_color: "#647E75".to_string(),
             height: 2751,
             id: 15110299,
             liked: false,
             photographer: "Efe Ersoy".to_string(),
             photographer_id: 393937500,
             photographer_url: "https://www.pexels.com/@efe-ersoy-393937500".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15110299/pexels-photo-15110299.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15110299/pexels-photo-15110299.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15110299/pexels-photo-15110299.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15110299/pexels-photo-15110299.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15110299/pexels-photo-15110299.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15110299/pexels-photo-15110299.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15110299/pexels-photo-15110299.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15110299/pexels-photo-15110299.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/to-eternity-15110299/".to_string(),
             width: 1830
        },
Photo {
             alt: "Fruity".to_string(),
             avg_color: "#85795F".to_string(),
             height: 3637,
             id: 15109951,
             liked: false,
             photographer: "Efe Ersoy".to_string(),
             photographer_id: 393937500,
             photographer_url: "https://www.pexels.com/@efe-ersoy-393937500".to_string(),
             src:  PhotoSrcSet {
                 landscape: "https://images.pexels.com/photos/15109951/pexels-photo-15109951.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=627&w=1200".to_string(),
                 large: "https://images.pexels.com/photos/15109951/pexels-photo-15109951.jpeg?auto=compress&cs=tinysrgb&h=650&w=940".to_string(),
                 large2x: "https://images.pexels.com/photos/15109951/pexels-photo-15109951.jpeg?auto=compress&cs=tinysrgb&dpr=2&h=650&w=940".to_string(),
                 medium: "https://images.pexels.com/photos/15109951/pexels-photo-15109951.jpeg?auto=compress&cs=tinysrgb&h=350".to_string(),
                 original: "https://images.pexels.com/photos/15109951/pexels-photo-15109951.jpeg".to_string(),
                 portrait: "https://images.pexels.com/photos/15109951/pexels-photo-15109951.jpeg?auto=compress&cs=tinysrgb&fit=crop&h=1200&w=800".to_string(),
                 small: "https://images.pexels.com/photos/15109951/pexels-photo-15109951.jpeg?auto=compress&cs=tinysrgb&h=130".to_string(),
                 tiny: "https://images.pexels.com/photos/15109951/pexels-photo-15109951.jpeg?auto=compress&cs=tinysrgb&dpr=1&fit=crop&h=200&w=280".to_string()
            },
             url: "https://www.pexels.com/photo/fruity-15109951/".to_string(),
             width: 2433
        }
    ],
      total_results: 8000,
    }
  }
}
