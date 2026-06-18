// Trait commun pour injection de dépendances
pub trait Vehicle {
    fn coords(&self) -> Vec<f64>;
    fn label(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Bus {
    pub ligne: String,
    pub arret: String,
    pub lat: f64,
    pub lon: f64,
}

impl Vehicle for Bus {
    fn coords(&self) -> Vec<f64> { vec![self.lat, self.lon] }
    fn label(&self) -> String { format!("Bus {} ({})", self.ligne, self.arret) }
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub vol: String,
    pub aeroport: String,
    pub lat: f64,
    pub lon: f64,
}

impl Vehicle for Plane {
    fn coords(&self) -> Vec<f64> { vec![self.lat, self.lon] }
    fn label(&self) -> String { format!("Vol {} ({})", self.vol, self.aeroport) }
}

#[derive(Debug, Clone)]
pub struct Car {
    pub plaque: String,
    pub lieu: String,
    pub lat: f64,
    pub lon: f64,
    pub avg_time: f64,
}

impl Vehicle for Car {
    fn coords(&self) -> Vec<f64> { vec![self.lat, self.lon] }
    fn label(&self) -> String {
        format!("{} ({}) [{} pts]", self.plaque, self.lieu, self.points())
    }
}

impl Car {
    // Système de points selon temps moyen
    pub fn points(&self) -> u32 {
        match self.avg_time {
            t if t < 10.0 => 100,
            t if t < 20.0 => 75,
            t if t < 30.0 => 50,
            t if t < 45.0 => 25,
            _ => 10,
        }
    }
}