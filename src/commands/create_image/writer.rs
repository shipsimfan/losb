use super::{
    error::CreateImageError,
    fat32::{bpb, calculate_size, fs_info, write_bpb, write_fs_info, FIXED_MEDIA, REMOVABLE_MEDIA},
    file::File,
    Cluster,
};
use crate::args::Options;

pub struct FileWriter<'a, 'b> {
    output: File,
    options: &'a Options<'b>,

    // FAT
    fat_size: usize,
    current_offset: usize,
    next_free_cluster: Cluster,
    free_count: usize,

    first_data_sector: usize,
    cluster_size: usize,
}

const END_OF_CLUSTER: Cluster = 0x0FFFFFFF;

impl<'a, 'b> FileWriter<'a, 'b> {
    pub fn new(fat_size: usize, options: &'a Options<'b>) -> Result<Self, CreateImageError> {
        // Calculate full file size from fat size
        let full_size = calculate_size(fat_size, options);

        // Create the output image
        let mut output = File::open_output(options.output_path())?;

        // Set the output images length
        output.set_len(full_size)?;

        // Write the BPB
        write_bpb(full_size, fat_size, &mut output, options)?;

        // Write the FSInfo
        write_fs_info(&mut output, options)?;

        // Zero the FATs
        let fat_offset = options.reserved_sectors() as usize * options.sector_size() as usize;
        output.seek(fat_offset)?;
        output.write_zeros(fat_size * options.num_fats() as usize)?;

        // Setup the writer
        output.seek(fat_offset)?;
        let mut writer = FileWriter {
            output,
            options,

            fat_size,
            current_offset: fat_offset,
            next_free_cluster: 0,
            free_count: fat_size / std::mem::size_of::<Cluster>(),

            first_data_sector: fat_offset + options.num_fats() as usize * fat_size,
            cluster_size: options.sector_size() as usize * options.sectors_per_cluster() as usize,
        };

        // Write the reserved clusters
        let mut first_cluster = END_OF_CLUSTER;
        first_cluster &= 0xFFFFFF00
            | if options.is_fixed_media() {
                FIXED_MEDIA
            } else {
                REMOVABLE_MEDIA
            } as u32;
        writer.write_cluster(first_cluster)?;
        writer.write_cluster(END_OF_CLUSTER)?;

        Ok(writer)
    }

    pub fn finalize(&mut self, root_cluster: Cluster) -> Result<(), CreateImageError> {
        // Write the root cluster
        bpb::write_root_cluster(&mut self.output, root_cluster)?;

        // Write the free cluster
        fs_info::write_free_cluster_info(
            &mut self.output,
            self.next_free_cluster,
            self.free_count,
            self.options,
        )
    }

    pub fn allocate_clusters(&mut self, count: usize) -> Result<Cluster, CreateImageError> {
        let first_cluster = self.next_free_cluster;

        for _ in 0..count - 1 {
            self.write_cluster(self.next_free_cluster + 1)?;
        }

        self.write_cluster(END_OF_CLUSTER)?;

        Ok(first_cluster)
    }

    pub fn write_data(&mut self, data: &[u8], cluster: Cluster) -> Result<(), CreateImageError> {
        self.output.seek(self.cluster_offset(cluster))?;

        self.output.write(&data)?;

        let remainder = self.cluster_size - (data.len() % self.cluster_size);
        if remainder != self.cluster_size {
            self.output.write_zeros(remainder)?;
        }

        Ok(())
    }

    fn cluster_offset(&self, cluster: Cluster) -> usize {
        (cluster as usize - 2) * self.cluster_size + self.first_data_sector
    }

    fn write_cluster(&mut self, value: Cluster) -> Result<(), CreateImageError> {
        for i in 0..self.options.num_fats() as usize {
            self.output.seek(self.current_offset + i * self.fat_size)?;
            self.output.write(&value.to_le_bytes())?;
        }

        self.current_offset += std::mem::size_of::<Cluster>();
        self.free_count -= 1;
        self.next_free_cluster += 1;

        Ok(())
    }
}
