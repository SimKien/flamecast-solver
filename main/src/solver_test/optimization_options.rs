use solver::CoolingSchedule;

pub type CoolingScheduleOption = (CoolingSchedule, f64);
pub type IterationCoolingOption = (CoolingScheduleOption, usize);

const LINEAR_COOLING_SCHEDULE1: CoolingSchedule = CoolingSchedule::Linear(0.01);
const LINEAR_COOLING_SCHEDULE2: CoolingSchedule = CoolingSchedule::Linear(0.002);

const LINEAR_COOLING1: CoolingScheduleOption = (LINEAR_COOLING_SCHEDULE1, 1.0);
const LINEAR_COOLING2: CoolingScheduleOption = (LINEAR_COOLING_SCHEDULE2, 1.0);

const LINEAR_OPTUONS1: IterationCoolingOption = (LINEAR_COOLING1, 100);
const LINEAR_OPTUONS2: IterationCoolingOption = (LINEAR_COOLING2, 500);

const FAST_CCOLING_SCHEDULE1: CoolingSchedule = CoolingSchedule::Fast(5.0);
const FAST_COOLING_SCHEDULE2: CoolingSchedule = CoolingSchedule::Fast(20.0);

const FAST_COOLING1: CoolingScheduleOption = (FAST_CCOLING_SCHEDULE1, 50.0);
const FAST_COOLING2: CoolingScheduleOption = (FAST_COOLING_SCHEDULE2, 1000.0);

const FAST_OPTUONS1: IterationCoolingOption = (FAST_COOLING1, 250);
const FAST_OPTUONS2: IterationCoolingOption = (FAST_COOLING2, 250);

const EXPONENTIAL_COOLING_SCHEDULE1: CoolingSchedule = CoolingSchedule::Exponential(0.92);
const EXPONENTIAL_COOLING_SCHEDULE2: CoolingSchedule = CoolingSchedule::Exponential(0.85);

const EXPONENTIAL_COOLING1: CoolingScheduleOption = (EXPONENTIAL_COOLING_SCHEDULE1, 10.0);
const EXPONENTIAL_COOLING2: CoolingScheduleOption = (EXPONENTIAL_COOLING_SCHEDULE2, 400.0);

const EXPONENTIAL_OPTUONS1: IterationCoolingOption = (EXPONENTIAL_COOLING1, 200);
const EXPONENTIAL_OPTUONS2: IterationCoolingOption = (EXPONENTIAL_COOLING2, 200);

const LOGARITHMIC_COOLING_SCHEDULE1: CoolingSchedule = CoolingSchedule::Logarithmic(80.0);
const LOGARITHMIC_COOLING_SCHEDULE2: CoolingSchedule = CoolingSchedule::Logarithmic(1.0);

const LOGARITHMIC_COOLING1: CoolingScheduleOption = (LOGARITHMIC_COOLING_SCHEDULE1, 20.0);
const LOGARITHMIC_COOLING2: CoolingScheduleOption = (LOGARITHMIC_COOLING_SCHEDULE2, 1.5);

const LOGARITHMIC_OPTUONS1: IterationCoolingOption = (LOGARITHMIC_COOLING1, 300);
const LOGARITHMIC_OPTUONS2: IterationCoolingOption = (LOGARITHMIC_COOLING2, 250);

pub const OPTIMIZATION_OPTIONS: [IterationCoolingOption; 8] = [
    LINEAR_OPTUONS1,
    LINEAR_OPTUONS2,
    FAST_OPTUONS1,
    FAST_OPTUONS2,
    EXPONENTIAL_OPTUONS1,
    EXPONENTIAL_OPTUONS2,
    LOGARITHMIC_OPTUONS1,
    LOGARITHMIC_OPTUONS2,
];
