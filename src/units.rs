
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct KilometrePerHour<T>(pub T) ;

#[derive(Serialize,Deserialize)]
pub struct MetrePerSec<T>(pub T) ;

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Celsius<T>(pub T) ;

#[derive(Serialize,Deserialize)]
pub struct Millibar<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Pascal<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct KiloPascal<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Percent<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Ratio<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Latitude<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Longitude<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Inch<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Millimetre<T>(pub T);


#[derive(Serialize,Deserialize)]
pub struct Metre<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Kilometre<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Rpm<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Hz<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Watt<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct KiloWatt<T>(pub T);


#[derive(Serialize,Deserialize)]
pub struct Millisecond<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Second<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Minute<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Hour<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Gram<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Kilogram<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct GramPerSec<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct LiterPerHour<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct MeterPerSecondSq<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct CentimeterPerSecondSq<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Newton<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct NewtonMetre<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Litre<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Millilitre<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Degree<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct DegreePerSecond<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct LiterPerHundredKm<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct MilliliterPerHundredKm<T>(pub T);

#[derive(Serialize,Deserialize)]
pub struct Volt<T>(pub T);


#[derive(Serialize,Deserialize)]
pub struct Amp<T>(pub T);


