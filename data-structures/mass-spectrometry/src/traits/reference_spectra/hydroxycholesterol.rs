//! Submodule providing data for the hydroxy-cholesterol molecule.

use crate::traits::SpectrumAlloc;

/// Trait for a spectrum of hydroxy-cholesterol.
pub trait HydroxyCholesterolSpectrum: SpectrumAlloc {
    /// Create a new spectrum of hydroxy-cholesterol.
    fn hydroxy_cholesterol() -> Self;
}

/// The precursor mass over charge value for hydroxy-cholesterol.
pub const HYDROXY_CHOLESTEROL_PRECURSOR_MZ: f32 = 385.345;

/// The mass over charge values for hydroxy-cholesterol.
pub const HYDROXY_CHOLESTEROL_MZ: [f32; 131] = [
    79.306_83, 81.036896, 83.170_62, 95.116_5, 97.141_31, 103.330635, 105.13884, 107.099525,
    109.088_67, 111.171_36, 117.067_56, 119.17308, 121.166855, 123.211_3, 125.549_04, 129.067_57,
    131.025_85, 133.06012, 135.10881, 137.158_2, 139.159_88, 143.095_25, 144.09375, 145.11795,
    147.135_38, 149.237, 151.340_97, 155.078_19, 156.218_5, 157.128, 159.123_2, 160.329_16,
    161.145, 163.198_15, 165.245_86, 167.088_36, 169.130_75, 170.001_28, 171.115_75, 173.205_08,
    175.229_89, 177.121_67, 179.243_74, 181.121_95, 182.870_96, 185.176_25, 187.167_39, 188.135_38,
    189.40036, 191.186_42, 193.249_73, 197.140_75, 199.207_49, 201.150_27, 202.018_68, 203.189_33,
    205.19397, 207.164_64, 209.172_24, 210.234_71, 211.334_08, 213.274_6, 214.419_53, 215.139_83,
    217.144_78, 218.287_45, 219.207_06, 221.182_13, 223.02652, 224.174_59, 225.354_8, 227.09613,
    229.292_11, 231.228_24, 233.224_27, 239.40976, 241.226_14, 243.170_8, 245.26358, 247.416_38,
    249.43924, 251.274_63, 253.427_61, 255.29483, 257.357_15, 259.194_2, 261.212_5, 263.759_25,
    267.261_4, 269.225_98, 271.266_14, 272.015_47, 273.283_5, 274.167_1, 283.198_18, 285.315_9,
    287.210_3, 288.743_62, 289.402_25, 291.233_92, 293.318_82, 295.074_13, 297.280_76, 299.349_8,
    301.309_2, 303.264_3, 305.25528, 309.267_73, 311.262_54, 313.286_38, 315.270_2, 324.4086,
    325.427_86, 327.317_96, 327.939_64, 329.279_4, 338.460_54, 339.46, 340.288_18, 341.369_42,
    343.326_35, 349.281_65, 354.272_03, 357.45163, 365.516_48, 367.329_04, 368.233_8, 368.874_88,
    384.324_3, 385.305_66, 386.437_32,
];
/// The intensities for hydroxy-cholesterol.
pub const HYDROXY_CHOLESTEROL_INTENSITIES: [f32; 131] = [
    4.979912, 22.765455, 16.608736, 84.772_97, 70.130226, 11.714973, 31.981926, 54.602894,
    91.455025, 46.116_69, 37.930_03, 74.307_09, 41.617023, 54.827194, 3.832052, 68.119_19,
    102.478645, 114.723_34, 107.749_02, 12.766851, 6.991966, 17.36504, 16.665106, 121.05883,
    66.699_09, 78.691_19, 21.242_49, 17.17906, 18.209969, 16.721523, 220.098_2, 7.839384,
    180.722_81, 176.15947, 85.956116, 18.90317, 36.619_06, 14.848515, 26.946314, 195.633_64,
    74.910_39, 184.579_04, 174.899_84, 63.308514, 11.29303, 37.871227, 325.631_16, 7.312701,
    98.873_01, 323.946_9, 71.510_2, 16.049942, 64.065384, 192.031_3, 14.154707, 120.749435,
    368.516_78, 226.5923, 28.575651, 15.240299, 6.454416, 176.759_49, 10.867487, 91.041_05,
    85.689_35, 7.409761, 365.31958, 28.595486, 40.717285, 11.677838, 11.328114, 145.452_4,
    80.116_07, 263.121_28, 252.111_95, 11.013158, 319.304_63, 56.70892, 139.755_74, 57.950_04,
    23.482685, 14.781034, 22.184134, 454.479_74, 154.093_49, 368.519_6, 56.954346, 42.744823,
    19.643261, 88.461494, 94.910_17, 16.442837, 413.641_63, 7.44519, 41.749924, 87.47644,
    128.763_34, 64.673_12, 14.391298, 13.502331, 75.397_29, 49.663_11, 169.150_27, 12.272851,
    152.579_4, 113.564_99, 30.968838, 16.352_49, 310.930_27, 41.870014, 20.039595, 72.416_03,
    155.088_06, 234.508_9, 24.093_28, 83.271_74, 53.767117, 85.915_54, 40.059822, 35.149338,
    25.068996, 30.07332, 9.299104, 60.239254, 6.935501, 14_848.888, 179.36615, 46.81424, 46.517662,
    447.722_3, 12.113778,
];

impl<S: SpectrumAlloc> HydroxyCholesterolSpectrum for S
where
    S::Mz: From<f32>,
    S::Intensity: From<f32>,
{
    fn hydroxy_cholesterol() -> Self {
        let mut spectrum = Self::with_capacity(
            HYDROXY_CHOLESTEROL_PRECURSOR_MZ.into(),
            HYDROXY_CHOLESTEROL_MZ.len(),
        );
        for (&mz, &intensity) in
            HYDROXY_CHOLESTEROL_MZ.iter().zip(HYDROXY_CHOLESTEROL_INTENSITIES.iter())
        {
            spectrum
                .add_peak(mz.into(), intensity.into())
                .expect("Failed to add hydroxy-cholesterol peak to spectrum");
        }
        spectrum
    }
}
