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
    query: Query<(Entity, &SpaceKit, Option<&Name>), Changed<SpaceKit>>,
    space_kit: Res<SpaceKitAssets>,
) {
    for (e, kit, name_maybe) in query.iter() {

        if name_maybe.is_none() {
            commands
            .entity(e)
            .insert(Name::new(format!("{:?}", kit)));
        }
        

        commands
            .entity(e)
            .insert(match kit {
                SpaceKit::Character(a) => match a {
                    Character::Alien => space_kit.alien.clone(),
                    Character::AstronautA => space_kit.astronaut_a.clone(),
                    Character::AstronautB => space_kit.astronaut_b.clone(),
                },
                SpaceKit::Barrel(a) => match a {
                    Barrel::Normal => space_kit.barrel.clone(),
                    Barrel::Multiple => space_kit.barrels.clone(),
                    Barrel::Rail => space_kit.barrels_rail.clone(),
                },
                SpaceKit::Bones => space_kit.bones.clone(),
                SpaceKit::Chimney(_) => todo!(),
                SpaceKit::Corridor(_) => todo!(),
                SpaceKit::Craft(_) => todo!(),
                SpaceKit::Crater(_) => todo!(),
                SpaceKit::Desk(a) => match a {
                    Desk::Chair => space_kit.desk_chair.clone(),
                    Desk::ChairArms => space_kit.desk_chair_arms.clone(),
                    Desk::ChairStool => space_kit.desk_chair_stool.clone(),
                    Desk::Computer => space_kit.desk_computer.clone(),
                    Desk::ComputerCorner => space_kit.desk_computer_corner.clone(),
                    Desk::ComputerScreen => space_kit.desk_computer_screen.clone(),
                },
                SpaceKit::Gate(_) => todo!(),
                SpaceKit::Hanger(_) => todo!(),
                SpaceKit::Machine(a) => match a {
                    Machine::Barrel => space_kit.machine_barrel.clone(),
                    Machine::BarrelLarge => space_kit.machine_barrel_large.clone(),
                    Machine::Generator => space_kit.machine_generator.clone(),
                    Machine::GeneratorLarge => space_kit.machine_generator_large.clone(),
                    Machine::Wireless => space_kit.machine_wireless.clone(),
                    Machine::WirelessCable => space_kit.machine_wireless_cable.clone(),
                },
                SpaceKit::Meteor(_) => todo!(),
                SpaceKit::Monorail(_) => todo!(),
                SpaceKit::Pipe(_) => todo!(),
                SpaceKit::Platform(_) => todo!(),
                SpaceKit::Rail(_) => todo!(),
                SpaceKit::Rock(_) => todo!(),
                SpaceKit::Rocket(a) => match a {
                    Rocket::BaseA => space_kit.rocket_base_a.clone(),
                    Rocket::BaseB => space_kit.rocket_base_b.clone(),
                    Rocket::FinsA => space_kit.rocket_fins_a.clone(),
                    Rocket::FinsB => space_kit.rocket_fins_b.clone(),
                    Rocket::FuelA => space_kit.rocket_fuel_a.clone(),
                    Rocket::FuelB => space_kit.rocket_fuel_b.clone(),
                    Rocket::SidesA => space_kit.rocket_sides_a.clone(),
                    Rocket::SidesB => space_kit.rocket_sides_b.clone(),
                    Rocket::TopA => space_kit.rocket_top_a.clone(),
                    Rocket::TopB => space_kit.rocket_top_b.clone(),
                },
                SpaceKit::Stairs(_) => todo!(),
                SpaceKit::SatelliteDish(_) => todo!(),
                SpaceKit::Supports(_) => todo!(),
                SpaceKit::Structure(_) => todo!(),
                SpaceKit::Terrain(_) => todo!(),
                SpaceKit::Turret(_) => todo!(),
                SpaceKit::Weapon(a) => match a {
                    Weapon::Gun => space_kit.weapon_gun.clone(),
                    Weapon::Rifle => space_kit.weapon_rifle.clone(),
                    Weapon::BlasterR => space_kit.weapon_blaster_r.clone(),
                },
                SpaceKit::Rover => todo!(),
            });
    }
}

#[derive(Component, PartialEq, Eq, Debug, Inspectable, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Character {
    Alien,
    #[default]
    AstronautA,
    AstronautB,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Barrel {
    #[default]
    Normal,
    Multiple,
    Rail,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Crater {
    #[default]
    Normal,
    Large,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Chimney {
    #[default]
    Normal,
    Detailed,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Desk {
    ChairArms,
    Chair,
    ChairStool,
    ComputerCorner,
    #[default]
    Computer,
    ComputerScreen,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Gate {
    Complex,
    #[default]
    Simple,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Machine {
    Barrel,
    BarrelLarge,
    #[default]
    Generator,
    GeneratorLarge,
    WirelessCable,
    Wireless,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Meteor {
    #[default]
    Normal,
    Detailed,
    Half,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Rail {
    Corner,
    End,
    #[default]
    Normal,
    Middle,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum SatelliteDish {
    Detailed,
    #[default]
    Normal,
    Large,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Stairs {
    Corner,
    #[default]
    Normal,
    Short,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Structure {
    #[default]
    Normal,
    Closed,
    Detailed,
    Diagonal,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Supports {
    High,
    #[default]
    Low,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Turret {
    Double,
    #[default]
    Single,
}

#[derive(Debug, PartialEq, Eq, Inspectable, Default, Copy, Clone)]
pub enum Weapon {
    #[default]
    Gun,
    Rifle,
    BlasterR,
}
