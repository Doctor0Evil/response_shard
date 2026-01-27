export type MaterialId = "PHA" | "BAGASSE";

export interface BiodegradationProfile {
  environment: "home_compost" | "industrial_compost" | "soil" | "marine";
  minDays: number;
  maxDays: number;
  notes: string;
}

export interface MaterialSafety {
  biobased: boolean;
  pfasFree: boolean;
  bpaFree: boolean;
  foodContactCertified: boolean;
  certifications: string[];
  nonToxicClaim: string;
}

export interface PerformanceProfile {
  maxUseTemperatureC: number;
  minUseTemperatureC: number;
  greaseResistance: "low" | "medium" | "high";
  moistureResistance: "low" | "medium" | "high";
  microwaveSafe: boolean;
  freezerSafe: boolean;
}

export interface LCABaseline {
  relativeGwpVsPE: number;
  relativeFossilDepletionVsPE: number;
  costRangeEurPerKg: [number, number];
  notes: string;
}

export interface TrayMaterial {
  id: MaterialId;
  name: string;
  description: string;
  safety: MaterialSafety;
  biodegradation: BiodegradationProfile[];
  performance: PerformanceProfile;
  lcaBaseline: LCABaseline;
}

export const PHA_MATERIAL: TrayMaterial = {
  id: "PHA",
  name: "Polyhydroxyalkanoate (PHA)",
  description:
    "Biobased, fully biodegradable family of polyesters produced by microbial fermentation, used as a higher-ceiling replacement for fossil plastics in trays and packaging.",
  safety: {
    biobased: true,
    pfasFree: true,
    bpaFree: true,
    foodContactCertified: true,
    certifications: [
      "TÜV OK Compost Home",
      "TÜV OK Compost Industrial",
      "Marine biodegradability certifications",
      "Food-contact compliance (various jurisdictions)"
    ],
    nonToxicClaim:
      "Multiple commercial grades (e.g., Nodax-type PHA) are documented as non-toxic to aquatic organisms, plants, and mammals under food-contact conditions."
  },
  biodegradation: [
    {
      environment: "soil",
      minDays: 180,
      maxDays: 365,
      notes:
        "Full biodegradation in wet soil and farmland typically occurs within about 6–12 months, without persistent microplastic fragments."
    },
    {
      environment: "home_compost",
      minDays: 180,
      maxDays: 365,
      notes: "Home compost degradation times are broadly similar to soil conditions."
    },
    {
      environment: "industrial_compost",
      minDays: 90,
      maxDays: 180,
      notes:
        "Elevated temperature and humidity shorten biodegradation times relative to ambient soil."
    },
    {
      environment: "marine",
      minDays: 180,
      maxDays: 365,
      notes:
        "Certified grades are reported to fully biodegrade in marine environments, although rates depend strongly on local conditions."
    }
  ],
  performance: {
    maxUseTemperatureC: 100,
    minUseTemperatureC: -20,
    greaseResistance: "high",
    moistureResistance: "high",
    microwaveSafe: true,
    freezerSafe: true
  },
  lcaBaseline: {
    relativeGwpVsPE: 5,
    relativeFossilDepletionVsPE: 5,
    costRangeEurPerKg: [1.18, 6.12],
    notes:
      "Current pilot-scale techno-environmental LCAs often show >5× GWP and fossil depletion vs polyethylene in stand-alone PHA configurations; tightly integrated waste-based systems can flip this comparison."
  }
};

export const BAGASSE_MATERIAL: TrayMaterial = {
  id: "BAGASSE",
  name: "Sugarcane Bagasse Pulp",
  description:
    "Molded pulp derived from sugarcane processing residue, used as a PFAS-free, compostable, fiber-based tray material for food packaging at scale.",
  safety: {
    biobased: true,
    pfasFree: true,
    bpaFree: true,
    foodContactCertified: true,
    certifications: [
      "OK Compost Home",
      "OK Compost Industrial",
      "EN13432",
      "ASTM D6400",
      "BPI",
      "FDA food-contact",
      "SGS food-contact",
      "ISO 9001",
      "ISO 14001",
      "BRC",
      "BSCI"
    ],
    nonToxicClaim:
      "Commercial sugarcane bagasse trays are marketed as PFAS-free, BPA-free, and food-safe for direct contact with meat, produce, and ready meals."
  },
  biodegradation: [
    {
      environment: "home_compost",
      minDays: 60,
      maxDays: 90,
      notes:
        "Bagasse trays fully decompose in roughly 60–90 days in home compost, returning to organic matter with no microplastic residues."
    },
    {
      environment: "industrial_compost",
      minDays: 45,
      maxDays: 60,
      notes:
        "Under controlled composting conditions, bagasse typically degrades faster than in home compost."
    },
    {
      environment: "soil",
      minDays: 60,
      maxDays: 120,
      notes:
        "In soil environments, degradation rates are similar to or slightly slower than home compost depending on moisture and temperature."
    }
  ],
  performance: {
    maxUseTemperatureC: 120,
    minUseTemperatureC: -10,
    greaseResistance: "high",
    moistureResistance: "high",
    microwaveSafe: true,
    freezerSafe: true
  },
  lcaBaseline: {
    relativeGwpVsPE: 0.6,
    relativeFossilDepletionVsPE: 0.4,
    costRangeEurPerKg: [0.7, 1.0],
    notes:
      "Because bagasse originates as sugarcane residue and uses renewable fiber with ISO-certified, water- and carbon-optimized plants, current LCAs consistently assign a lower near-term eco-impact than PHA for tray applications."
  }
};

export const MATERIALS: Record<MaterialId, TrayMaterial> = {
  PHA: PHA_MATERIAL,
  BAGASSE: BAGASSE_MATERIAL
};
