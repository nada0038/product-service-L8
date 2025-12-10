use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 65\" 4K UHD Smart TV".to_string(),
            price: 799.99,
            description: "Experience stunning picture quality with this 65-inch Samsung 4K UHD Smart TV. Features HDR, voice control, and streaming apps built-in.".to_string(),
            image: "/Samsung tv.avif".to_string()
        },
        Product {
            id: 2,
            name: "Apple iPhone 15 Pro 256GB".to_string(),
            price: 1099.99,
            description: "The latest iPhone with A17 Pro chip, Pro camera system, and titanium design. 256GB storage capacity.".to_string(),
            image: "/Iphone 15 pro.jpeg".to_string()
        },
        Product {
            id: 3,
            name: "Sony WH-1000XM5 Wireless Headphones".to_string(),
            price: 399.99,
            description: "Industry-leading noise cancellation with exceptional sound quality. 30-hour battery life and quick charge feature.".to_string(),
            image: "/sony headphone.jpg".to_string()
        },
        Product {
            id: 4,
            name: "MacBook Pro 14\" M3 Chip".to_string(),
            price: 1999.99,
            description: "Powerful 14-inch MacBook Pro with M3 chip, 16GB RAM, and 512GB SSD. Perfect for professionals and creatives.".to_string(),
            image: "/macbook.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Nintendo Switch OLED Console".to_string(),
            price: 349.99,
            description: "Nintendo Switch with vibrant 7-inch OLED screen. Play at home or on the go with detachable Joy-Con controllers.".to_string(),
            image: "/nintendo.avif".to_string()
        },
        Product {
            id: 6,
            name: "Canon EOS R6 Mark II Camera".to_string(),
            price: 2499.99,
            description: "Professional mirrorless camera with 24.2MP full-frame sensor, 4K video recording, and advanced autofocus system.".to_string(),
            image: "/Canon Eos.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Dyson V15 Detect Cordless Vacuum".to_string(),
            price: 749.99,
            description: "Powerful cordless vacuum with laser technology to reveal microscopic dust. Up to 60 minutes of runtime.".to_string(),
            image: "/Dyson Vacum.webp".to_string()
        },
        Product {
            id: 8,
            name: "AirPods Pro (2nd Generation)".to_string(),
            price: 249.99,
            description: "Active Noise Cancellation, spatial audio, and MagSafe charging case. Up to 6 hours of listening time.".to_string(),
            image: "/Airpods pro.jpg".to_string()
        },
        Product {
            id: 9,
            name: "LG 27\" UltraGear Gaming Monitor".to_string(),
            price: 299.99,
            description: "27-inch 4K gaming monitor with 144Hz refresh rate, 1ms response time, and NVIDIA G-SYNC compatibility.".to_string(),
            image: "/LG monitor.avif".to_string()
        },
        Product {
            id: 10,
            name: "Xbox Series X Console".to_string(),
            price: 499.99,
            description: "Next-generation gaming console with 4K gaming, ray tracing, and fast load times. Includes one wireless controller.".to_string(),
            image: "/Xbox.jpg".to_string()
        }
    ]
}