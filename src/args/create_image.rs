use super::{ArgumentError, Options};
use argparse::ArgumentParser;
use std::path::PathBuf;

pub fn add_create_image_arguments(parser: &mut ArgumentParser<Options>) {
    add_output_path_argument(parser);
    add_sector_size_argument(parser);
    add_sectors_per_cluster_argument(parser);
    add_reserved_sectors_argument(parser);
    add_num_fats_argument(parser);
    add_fixed_argument(parser);
    add_removable_argument(parser);
    add_volume_id_argument(parser);
}

fn add_output_path_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--output", |args, options| {
            Ok(options.set_output_path(PathBuf::from(&args[0])))
        })
        .name("-o")
        .help("Sets the file to output the image to")
        .required(false)
        .count(1);
}

fn add_sector_size_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--sector-size", |args, options| match args[0].parse() {
            Ok(sector_size) => match [512, 1024, 2048, 4096].contains(&sector_size) {
                true => Ok(options.set_sector_size(sector_size)),
                false => Err(Box::new(ArgumentError::InvalidSectorSize(args[0].clone()))),
            },
            Err(_) => Err(Box::new(ArgumentError::InvalidSectorSize(args[0].clone()))),
        })
        .help("Sets the sector size for the image")
        .required(false)
        .count(1);
}

fn add_sectors_per_cluster_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--cluster-size", |args, options| {
            match args[0].parse::<u8>() {
                Ok(sectors_per_cluster) => match sectors_per_cluster.is_power_of_two() {
                    true => Ok(options.set_sectors_per_cluster(sectors_per_cluster)),
                    false => Err(Box::new(ArgumentError::InvalidClusterSize(args[0].clone()))),
                },
                Err(_) => Err(Box::new(ArgumentError::InvalidClusterSize(args[0].clone()))),
            }
        })
        .help("Sets the sectors per cluster for the image")
        .required(false)
        .count(1);
}

fn add_reserved_sectors_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--reserved-sectors", |args, options| {
            match args[0].parse::<u16>() {
                Ok(reserved_sectors) => match reserved_sectors >= 2 {
                    true => Ok(options.set_reserved_sectors(reserved_sectors)),
                    false => Err(Box::new(ArgumentError::InvalidReservedSectors)),
                },
                Err(_) => Err(Box::new(ArgumentError::InvalidReservedSectors)),
            }
        })
        .help("Sets the sectors per cluster for the image")
        .required(false)
        .count(1);
}

fn add_num_fats_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--num-fats", |args, options| match args[0].parse::<u8>() {
            Ok(num_fats) => match num_fats > 0 {
                true => Ok(options.set_num_fats(num_fats)),
                false => Err(Box::new(ArgumentError::InvalidNumFats)),
            },
            Err(_) => Err(Box::new(ArgumentError::InvalidNumFats)),
        })
        .help("Sets the number of FATs in the image")
        .required(false)
        .count(1);
}

fn add_fixed_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--fixed", |_, options| Ok(options.set_fixed_media()))
        .help("Builds the image for fixed media")
        .required(false)
        .count(0);
}

fn add_removable_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument(
            "--removable",
            |_, options| Ok(options.set_removable_media()),
        )
        .help("Builds the image for removable media")
        .required(false)
        .count(0);
}

fn add_volume_id_argument(parser: &mut ArgumentParser<Options>) {
    parser
        .add_argument("--volume-id", |args, options| match args[0].parse() {
            Ok(volume_id) => Ok(options.set_volume_id(volume_id)),
            Err(_) => Err(Box::new(ArgumentError::InvalidVolumeID)),
        })
        .help("Sets the volume ID of the image")
        .required(false)
        .count(1);
}
