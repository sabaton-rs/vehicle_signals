
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct KilometrePerHour<T>(pub T) ;

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct MetrePerSec<T>(pub T) ;

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Celsius<T>(pub T) ;

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Millibar<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Pascal<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct KiloPascal<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Percent<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Ratio<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Latitude<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Longitude<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Inch<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Millimetre<T>(pub T);


#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Metre<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Kilometre<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct RPM<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Hertz<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Watt<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Kilowatt<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct KilowattHour<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Millisecond<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Second<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Minute<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Hour<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Gram<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Kilogram<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct GramPerSec<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct LiterPerHour<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct MeterPerSecondSq<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct CentimeterPerSecondSq<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Newton<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct NewtonMetre<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Litre<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Millilitre<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Degree<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct DegreePerSecond<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct LiterPerHundredKm<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct MilliliterPerHundredKm<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Volt<T>(pub T);

#[derive(Serialize,Deserialize, Default, Clone)]
pub struct Amp<T>(pub T);


