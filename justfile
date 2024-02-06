loop_device := `sudo losetup -f`

default:
    @just --list

clean:
    -test -e .out && rm -r .out
    mkdir .out

    -test -e .bootable-images || mkdir .bootable-images
    -test -e .docker-images   || mkdir .docker-images

build-fs: clean
    @# Make all the dirs
    test -d .out/fs || mkdir  .out/fs
    mkdir  .out/fs/bin
    mkdir  .out/fs/boot
    mkdir  .out/fs/boot/grub
    mkdir  .out/fs/lib
    mkdir  .out/fs/lib/x86_64-linux-gnu
    mkdir  .out/fs/lib64
    mkdir  .out/fs/sbin

    @# Copy the things that GRUB needs
    cp deps/.out/linux.x86  .out/fs/boot/linux.x86
    cp grub.cfg             .out/fs/boot/grub/grub.cfg

    #@ Copy files that init links to
    cp /lib/x86_64-linux-gnu/libgcc_s.so.1  .out/fs/lib/x86_64-linux-gnu/libgcc_s.so.1
    cp /lib/x86_64-linux-gnu/libc.so.6      .out/fs/lib/x86_64-linux-gnu/libc.so.6
    cp /lib64/ld-linux-x86-64.so.2          .out/fs/lib64/ld-linux-x86-64.so.2

    @# Compile all modules
    just modules/build
    -cp modules/.out/bin/*   .out/fs/bin
    -cp modules/.out/sbin/*  .out/fs/sbin

    @# Make link to init
    ln .out/fs/sbin/quartz .out/fs/sbin/init

create-bootable-image name="devel" size="1024": build-fs
    test {{size}} -ge 128
    dd if=/dev/zero of=.out/oxide.img bs=1MiB count={{size}}

    sudo losetup {{loop_device}} .out/oxide.img

    sudo parted -s {{loop_device}} -- \
        mklabel gpt \
        disk_set pmbr_boot on \
        mkpart esp 1MiB 99MiB \
        set 1 esp on \
        mkpart oxide 100MiB $(expr {{size}} - 1)MiB

    mkdir .out/mnt_esp
    mkdir .out/mnt_oxide

    sudo mkfs.fat -F 32 {{loop_device}}p1
    sudo mount {{loop_device}}p1 .out/mnt_esp

    sudo mkfs.ext4 {{loop_device}}p2
    sudo mount {{loop_device}}p2 .out/mnt_oxide

    sudo cp -r .out/fs/* .out/mnt_oxide

    OXIDE_WD=$PWD && cd deps/.out/grub && sudo ./grub-install --target x86_64-efi \
        --directory=grub-core \
        --efi-directory=$OXIDE_WD/.out/mnt_esp/ \
        --bootloader-id=GRUB \
        --modules="normal part_gpt linux" \
        --fonts="unicode" \
        --boot-directory=$OXIDE_WD/.out/mnt_oxide/boot \
        --no-floppy --removable \
        {{loop_device}}p1

    sudo umount .out/mnt_esp
    sudo umount .out/mnt_oxide

    sudo losetup -d {{loop_device}}

    mv .out/oxide.img .images/{{name}}.oxide.img

test-bootable-image image="devel":
    qemu-system-x86_64 -nographic -m 2g -bios /usr/share/ovmf/OVMF.fd -drive format=raw,file=.bootable-images/{{image}}.oxide.img

test-new-bootable-image name="devel" size="1024": (create-bootable-image name size) (test-bootable-image name)