use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use iyes_loopless::prelude::*;

use crate::{assets::SpaceKitAssets, GameState};

pub struct SpaceKitPlugin;

impl Plugin for SpaceKitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_spacekit.run_in_state(GameState::Playing))
            .register_inspectable::<SpaceKit>();
    }
}

fn spawn_spacekit(
    mut commands: Commands,
    query: Query<(Entity, &SpaceKit), Changed<SpaceKit>>,
    spacekit: Res<SpaceKitAssets>,
) {
    for (e, kit) in query.iter() {
        commands
            .entity(e)
            .insert(Name::new(format!("{:?}", kit)))
            .insert(match kit {
                SpaceKit::Character(a) => match a {
                    Character::Alien => spacekit.alien.clone(),
                    Character::AstronautA => spacekit.astronaut_a.clone(),
                    Character::AstronautB => spacekit.astronaut_b.clone(),
                },
                SpaceKit::Barrel(a) => {
                    match a {
                        Barrel::Normal => spacekit.barrel.clone(),
                        Barrel::Multiple => spacekit.barrels.clone(),
                        Barrel::Rail => spacekit.barrels_rail.clone(),
                    }
                },
                SpaceKit::Bones => spacekit.bones.clone(),
                SpaceKit::Chimney(_) => todo!(),
                SpaceKit::Corridor(_) => todo!(),
                SpaceKit::Craft(_) => todo!(),
                SpaceKit::Crater(_) => todo!(),
                SpaceKit::Desk(a) => match a {
                    Desk::Chair => spacekit.desk_chair.clone(),
                    Desk::ChairArms => spacekit.desk_chair_arms.clone(),
                    Desk::ChairStool => spacekit.desk_chair_stool.clone(),
                    Desk::Computer => spacekit.desk_computer.clone(),
                    Desk::ComputerCorner => spacekit.desk_computer_corner.clone(),
                    Desk::ComputerScreen => spacekit.desk_computer_screen.clone(),
                },
                SpaceKit::Gate(_) => todo!(),
                SpaceKit::Hanger(_) => todo!(),
                SpaceKit::Machine(a) => match a {
                    Machine::Barrel => spacekit.machine_barrel.clone(),
                    Machine::BarrelLarge => spacekit.machine_barrel_large.clone(),
                    Machine::Generator => spacekit.machine_generator.clone(),
                    Machine::GeneratorLarge => spacekit.machine_generator_large.clone(),
                    Machine::Wireless => spacekit.machine_wireless.clone(),
                    Machine::WirelessCable => spacekit.machine_wireless_cable.clone(),
                },
                SpaceKit::Meteor(_) => todo!(),
                SpaceKit::Monorail(_) => todo!(),
                SpaceKit::Pipe(_) => todo!(),
                SpaceKit::Platform(_) => todo!(),
                SpaceKit::Rail(_) => todo!(),
                SpaceKit::Rock(_) => todo!(),
                SpaceKit::Rocket(_) => todo!(),
                SpaceKit::Stairs(_) => todo!(),
                SpaceKit::SatelliteDish(_) => todo!(),
                SpaceKit::Supports(_) => todo!(),
                SpaceKit::Structure(_) => todo!(),
                SpaceKit::Terrain(_) => todo!(),
                SpaceKit::Turret(_) => todo!(),
                SpaceKit::Weapon(a) => match a {
                    Weapon::Gun => spacekit.weapon_gun.clone(),
                    Weapon::Rifle => spacekit.weapon_rifle.clone(),
                    Weapon::BlasterR => spacekit.weapon_blaster_r.clone(),
                },
                SpaceKit::Rover => todo!(),
            });
    }
}

#[derive(Component, PartialEq, Debug, Inspectable, Copy, Clone)]
pub enum SpaceKit {
    Character(Character),
    Barrel(Barrel),
    Bones,
    Chimney(Chimney),
    Corridor(Corridor),
    Craft(Craft),
    Crater(Crater),
    Desk(Desk),
    Gate(Gate),
    Hanger(Hanger),
    Machine(Machine),
    Meteor(Meteor),
    Monorail(Monorail),
    Pipe(Pipe),
    Platform(Platform),
    Rail(Rail),
    Rock(Rock),
    Rocket(Rocket),
    Stairs(Stairs),
    SatelliteDish(SatelliteDish),
    Supports(Supports),
    Structure(Structure),
    Terrain(Terrain),
    Turret(Turret),
    Weapon(Weapon),
    Rover,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Character {
    Alien,
    #[default]
    AstronautA,
    AstronautB,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Barrel {
    #[default]
    Normal,
    Multiple,
    Rail,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Corridor {
    #[default]
    Normal,
    Open,
    Corner,
    CornerRound,
    CornerRoundWindow,
    Cross,
    Detailed,
    End,
    Roof,
    Split,
    WallCorner,
    Wall,
    WindowClosed,
    Window,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Craft {
    CargoA,
    CargoB,
    Miner,
    Racer,
    #[default]
    SpeederA,
    SpeederB,
    SpeederC,
    SpeederD,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Crater {
    #[default]
    Normal,
    Large,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Chimney {
    #[default]
    Normal,
    Detailed,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Desk {
    ChairArms,
    Chair,
    ChairStool,
    ComputerCorner,
    #[default]
    Computer,
    ComputerScreen,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Gate {
    Complex,
    #[default]
    Simple,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Hanger {
    LargeA,
    #[default]
    LargeB,
    RoundA,
    RoundB,
    RoundGlass,
    SmallA,
    SmallB,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Machine {
    Barrel,
    BarrelLarge,
    #[default]
    Generator,
    GeneratorLarge,
    WirelessCable,
    Wireless,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Meteor {
    #[default]
    Normal,
    Detailed,
    Half,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Monorail {
    TrackCornerLarge,
    TrackCornerSmall,
    TrackSlope,
    #[default]
    TrackStraight,
    TrackSupportCorner,
    TrackSupport,
    TrainBox,
    TrainCargo,
    TrainEnd,
    TrainFlat,
    TrainFront,
    TrainPassenger,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Pipe {
    CornerDiagonal,
    Corner,
    CornerRound,
    CornerRoundLarge,
    Cross,
    End,
    Entrance,
    #[default]
    Open,
    RampLarge,
    RampSmall,
    Ring,
    RingHighEnd,
    RingHigh,
    RingSupport,
    Split,
    Straight,
    SupportHigh,
    SupportLow,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Platform {
    #[default]
    Center,
    Corner,
    CornerOpen,
    CornerRound,
    End,
    High,
    Large,
    Long,
    Low,
    Side,
    SmallDiagonal,
    Small,
    Straight,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Rail {
    Corner,
    End,
    #[default]
    Normal,
    Middle,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Rock {
    #[default]
    Normal,
    LargeA,
    LargeB,
    SmallA,
    SmallB,
    Crystals,
    CrystalsLargeA,
    CrystalsLargeB,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Rocket {
    #[default]
    BaseA,
    BaseB,
    FinsA,
    FinsB,
    FuelA,
    FuelB,
    SidesA,
    SidesB,
    TopA,
    TopB,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum SatelliteDish {
    Detailed,
    #[default]
    Normal,
    Large,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Stairs {
    Corner,
    #[default]
    Normal,
    Short,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Structure {
    #[default]
    Normal,
    Closed,
    Detailed,
    Diagonal,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Supports {
    High,
    #[default]
    Low,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Terrain {
    #[default]
    Normal,
    Ramp,
    RampLargeDetailed,
    RampLarge,
    RoadCorner,
    RoadCross,
    RoadEnd,
    RoadSplit,
    RoadStraight,
    SideCliff,
    SideCorner,
    SideCornerInner,
    SideEnd,
    Side,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Turret {
    Double,
    #[default]
    Single,
}

#[derive(Debug, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum Weapon {
    #[default]
    Gun,
    Rifle,
    BlasterR
}
