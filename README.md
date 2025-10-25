# 🌍 Henley Passport Index Viewer

A modern, desktop application built with Rust and egui that provides an interactive interface to explore the Henley Passport Index data. View global passport rankings and search visa requirements between countries.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

## ✨ Features

### 🏆 Passport Rankings
- View comprehensive global passport power rankings
- Color-coded badges for top 3 countries (Gold, Silver, Bronze)
- Real-time search and filter by country name
- Displays visa-free access count for each country
- Clean, striped table layout for easy reading

### 🔍 Visa Requirements Search
- Select origin country (your passport)
- Optional destination country filter
- Interactive dropdown with search functionality
- Color-coded visa requirement badges:
  - ✓ **Visa Free** (Green)
  - ✗ **Visa Required** (Red)
  - ◉ **Visa on Arrival** (Orange)
  - ◎ **E-Visa** (Teal)
- Scrollable results with detailed information

### 🎨 Modern UI Design
- Clean, professional interface with consistent light theme
- Responsive layout with smooth scrolling
- Enhanced typography and spacing
- Rounded corners and subtle shadows
- Color-coded elements for better visual hierarchy

## 📋 Prerequisites

- **Rust** (1.70 or higher) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo** (comes with Rust)

## 🚀 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/henley-passport-index.git
   cd henley-passport-index
   ```

2. **Add dependencies to `Cargo.toml`**
   ```toml
   [dependencies]
   eframe = "0.28"
   egui = "0.28"
   serde = { version = "1.0", features = ["derive"] }
   csv = "1.3"
   ```

3. **Prepare the data files**
   
   Place these CSV files in your project root:
   - `henley-passport-index-count-2025-10-17.csv` - Summary data with visa-free counts
   - `henley-passport-index-2025-10-17.csv` - Detailed visa requirements

   **Expected CSV format:**

   **Summary file** (`henley-passport-index-count-2025-10-17.csv`):
   ```csv
   Origin,Visa Free,Visa Required
   Singapore,193,32
   South Korea,190,35
   Japan,189,36
   ```

   **Details file** (`henley-passport-index-2025-10-17.csv`):
   ```csv
   Origin,Destination,Requirement
   Singapore,Japan,visa free
   Singapore,United States,visa free
   Singapore,Russia,e-visa
   ```

4. **Build and run**
   ```bash
   cargo run --release
   ```

## 📦 Project Structure

```
henley-passport-index/
├── src/
│   └── main.rs                                          # Main application code
├── henley-passport-index-count-2025-10-17.csv          # Summary data
├── henley-passport-index-2025-10-17.csv                # Detailed visa data
├── Cargo.toml                                           # Rust dependencies
└── README.md                                            # This file
```

## 🎯 Usage

### Viewing Rankings
1. Launch the application
2. Click on the **🏆 Ranking** tab (default view)
3. Use the search box to filter countries
4. Scroll through the complete rankings
5. Top 3 countries are highlighted with colored badges

### Searching Visa Requirements
1. Click on the **🔍 Search** tab
2. Select your passport country from the **Origin Country** dropdown
3. (Optional) Select a specific destination country to filter results
4. View visa requirements with color-coded badges
5. Scroll through all destinations

## 🛠️ Technical Details

### Built With
- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[egui](https://github.com/emilk/egui)** - Immediate mode GUI library
- **[eframe](https://github.com/emilk/egui/tree/master/crates/eframe)** - egui framework for native apps
- **[serde](https://serde.rs/)** - Serialization/deserialization framework
- **[csv](https://github.com/BurntSushi/rust-csv)** - CSV parsing library

### Key Features Implementation
- **Fixed Light Theme**: Ignores system dark/light mode preferences
- **In-Memory Data**: Fast performance with HashMap-based lookups
- **Responsive Design**: Adapts to window resizing (min: 900x600, default: 1200x800)
- **Efficient Rendering**: Uses egui's immediate mode rendering for smooth updates

## 🎨 Customization

### Changing Colors
Edit the RGB values in the `update` function:

```rust
// Header color
.fill(Color32::from_rgb(41, 98, 255))  // Blue header

// Background color
.fill(Color32::from_rgb(248, 249, 252))  // Light gray background
```

### Modifying Window Size
Adjust in the `main` function:

```rust
.with_inner_size([1200.0, 800.0])      // Default size
.with_min_inner_size([900.0, 600.0])   // Minimum size
```

## 📊 Data Sources

This application uses data from the **Henley Passport Index**, which ranks global passports according to the number of destinations their holders can access without a prior visa.

- Data current as of October 17, 2025
- Covers visa requirements for 195+ countries/territories
- Updated regularly by Henley & Partners

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 🐛 Known Issues

- Emoji rendering may not work on all systems (already replaced with Unicode symbols)
- CSV files must be in the exact format specified
- Large datasets (500+ countries) may impact performance

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Henley & Partners** for providing the passport index data
- **egui community** for the excellent GUI framework
- **Rust community** for the amazing language and ecosystem

## 📧 Contact

Your Name - [@ChainedAlchemy](https://x.com/ChainedAlchemy)

Project Link: [Here](https://github.com/elysiumor/fictional-journey)

---

**⭐ If you find this project useful, please consider giving it a star!**

## 🔮 Future Enhancements

- [ ] Export results to PDF/CSV
- [ ] Historical data comparison
- [ ] Interactive charts and graphs
- [ ] Multi-language support
- [ ] Dark theme toggle
- [ ] Real-time data updates from API
- [ ] Passport strength comparison tool
- [ ] Travel planning assistant
