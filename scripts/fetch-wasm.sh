#!/usr/bin/env bash

# you can see the updates at https://kusama.subscan.io/event?module=system&event=codeupdated&address=&startDate=&endDate=

# export SRV=localhost
# export KUSAMA=http://$SRV:9933
# export POLKADOT=http://$SRV:9934

mkdir -p \
    data/polkadot/V12 data/polkadot/V13 data/polkadot/V14 \
    data/kusama/V12 data/kusama/V13 data/kusama/V14 \
    data/westend/V12 data/westend/V13 data/westend/V14

# Kusama
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x476801cf6aac6ed4f8d782c96b450ba9444b545459fe0a30e1cea99c4dba4420 --output kusama/kusama-1050.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x149c7c293131ff9ad6bc531ec77e14aa40a9174cdf891948ea71c13578c8174e --output kusama/kusama-1051.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0xfc3ff2eff20a2884ae407a16ddcc40813394c162f7af839396923a8c976bc07b --output kusama/kusama-1052.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x074cbe7edbac908fe17ffddbe04cc2bede96663cf99c80ce44e38677025b2d46 --output kusama/kusama-1053.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x3fa6a530850324391fde50bdf0094bdc17ee17ec84aca389b4047ef54fea0037 --output kusama/kusama-1054.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0xf97657e864547b22004581f9b10f63f8ac56fad6ccefab139a05b7d6543e04a6 --output kusama/kusama-1055.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x4b8928d922b8940fe2f8a80d3491ab5992dd610d498dafe28445fa1ba7405f5b --output kusama/kusama-1058.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x478f359d51519b954a49567ae01afd2d62b315bd06d94e433fba212c8a7eef76 --output kusama/kusama-1062.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0xa39d5ce01a01331dc5322ba8ad0231fe88809f588c220a4d832f417cd5f7f516 --output kusama/kusama-2005.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x5b841f07449bb0c1779cb6559700582aa5ee542bc3403394fe1e04f71070cc32 --output kusama/kusama-2007.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x715dbf4012cdca810bcb2dca507d856e3fa719f3cf072058a2be378fd3aedeeb --output kusama/kusama-2008.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0xf7b11a9fbbd5390c96891bdcffcea4560369ed2068b69af03136c425cf15de74 --output kusama/kusama-2011.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x10dd21e188e6f32fec85d1561c2abf2d741e4245217ba3ba6e85efc6217a5012 --output kusama/kusama-2012.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0xd49d9570988b8b4c5f1b3f007b0e5c9c41e9316126707429ba0bfb60220f3e38 --output kusama/kusama-2013.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0xc454bb2fecffcc658c23db4d566b09b9744f1bd1d2227449f1803b2209b41524 --output kusama/kusama-2015.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x26de944a68c189878cf4bb93ba96af6f285ed164a14db7b221bfc6ceeeea5359 --output kusama/kusama-2019.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x6e4d2ab10da6c7834d37e6ab73a23fc5dda3d265e89e6b9b6cf6c5d62ce77a49 --output kusama/kusama-2022.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0xb516f157fc9e346b41cb6baddf3da2b38556c0d3d38c1ddaa25a07c76fb4a984 --output kusama/kusama-2023.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x59f499676353e5dd4534e5f0aef792c0a1e58df2ab882bcea702b975301b382f --output kusama/kusama-2024.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0xc5a6514d0a03a0c917133e34479f2d8b1f38bbaf526e9ea1ec241d66923f430a --output kusama/kusama-2026.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x32bf2564b651834a62ae3474d3f425803eb56c4a368fb6e74ed3e4f87c512760 --output kusama/kusama-2027.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x6f3f5cf507a83d05cb6339273d57517972d5e7482f192f107eacd0356f286a87 --output kusama/kusama-2028.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x2a2de75ef144e553ef0267091e42cd9863a761751bc7dbf8343e98c9edb0e14e --output kusama/kusama-2029.wasm
# cargo run --profile production -p subwasm -- get --url $KUSAMA --block 0x0c3743f02f149d2513784ee48648c6e2b5be2b0cf2c477c34de4c7b2b74eb285 --output kusama/kusama-2030.wasm

# Polkadot
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x4d376613c5ef02fc927a99cdac48b2f16ad2964cde6b52af7e8648eca29c308b --output polkadot/polkadot-01.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x66917aa154172f1a5a2ba8fde1c67cce45cc58f1f43b22f7de795968a536945d --output polkadot/polkadot-05.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x6c1c3ff09892c59a40086230309feb69d00db5d5ac464fe3ca0a44ef7cf76724 --output polkadot/polkadot-06.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x110a49b52e09da74d772d178c8e1d57d7dbf53b94af025b387c3b6e1161afa44 --output polkadot/polkadot-07.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x05eb07d6f133ba431adcc35bff2d4609aa81852dd2e029fc1d362272bfc7071f --output polkadot/polkadot-08.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0xee64472183796404ac1943d58cd554c40ec3b3bf57c1798b713289baef7e0d98 --output polkadot/polkadot-09.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x73ed90590022187998dfb260fc98cb4705092d4ed227368776e5ccba450c10a7 --output polkadot/polkadot-10.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x7e98cc466d39e7ed3a66d4751b549a35234e83f4770211d00dd59f3bf522c5c7 --output polkadot/polkadot-11.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x1e72be5704e0ad2219580d3a171621688f451711fe3ddeb453a44fa00402ae64 --output polkadot/polkadot-12.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x89f7032b1971c98b78960ec31712e16d0b276f8ef4940055046a05e03a373784 --output polkadot/polkadot-13.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x36d0394c5e75cf4f5d4126251e1d3e2a61dc2f6691cdb56770869a04fd622a49 --output polkadot/polkadot-14.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x07b3a72139c52830f6b7a6c04ab526fbfd035939cf1cf095022a77f2636df0be --output polkadot/polkadot-15.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0xac90171015f56dd0b0326e9bc510c17b03ba0d234bae31e633efe9f913874f2b --output polkadot/polkadot-16.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0xe5863fc3ee8d0391d7771bd027861e94eed0bd1a388df0d663a52fdeef962a99 --output polkadot/polkadot-17.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x10a60c95221396d9ad9cca41baeb32cde2ebb6dff0fbd04dbbd5a3bfdea2a345 --output polkadot/polkadot-18.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x5cffdbe008ff378eb9b93817caa1bc9005a9391cf01ed21425c1e9b523c4cc38 --output polkadot/polkadot-23.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x539cf843cc3e6617a14c2bfb4602ffb946c93adaeafc3e0fb425ca86c151f559 --output polkadot/polkadot-24.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x08aad1540d95e5aedf7dc65c627b4c7094626b78512a74ce482a82fd1beca5fc --output polkadot/polkadot-25.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x06c56436f57307ca74b363a2b9e0664ee49f041bc1387b2df1592ecfb9219ede --output polkadot/polkadot-26.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x58d7d68f73e490feca540dabd9ada1304930713860dae06e27410873078f3133 --output polkadot/polkadot-27.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x95488d035a54f89a451f32ebcd8b90dbc792713a835dcaf719cb8d3b0f578aca --output polkadot/polkadot-28.wasm
# cargo run --profile production -p subwasm -- get --url $POLKADOT --block 0x7f4c316fdd0780a5d3ae8f0a6f33e27aefeb9d074bed1dad5f5ab8baf42311b7 --output polkadot/polkadot-29.wasm

function runtime_to_semver {
    minor=${1:0:1}
    patch=${1:1:2}
    # echo $minor $patch
    printf "0.%d.%d\n" $minor $patch
}

for chain in polkadot kusama; do
    for version in 9000; do
        echo $chain $version
        semver=$(runtime_to_semver $version )
        url=https://github.com/paritytech/polkadot/releases/download/v$semver/${chain}_runtime-v$version.compact.wasm
        # echo $url
        wget $url -O data/$chain/V12/$version.wasm
    done
done

for chain in polkadot kusama; do
    for version in 9090; do
        echo $chain $version
        semver=$(runtime_to_semver $version )
        url=https://github.com/paritytech/polkadot/releases/download/v$semver/${chain}_runtime-v$version.compact.compressed.wasm
        # echo $url
        wget $url -O data/$chain/V13/$version.wasm
    done
done

for chain in polkadot kusama; do
    for version in 9100 9260 9270 9280 9290 9291 9300; do
        echo $chain $version
        semver=$(runtime_to_semver $version )
        url=https://github.com/paritytech/polkadot/releases/download/v$semver/${chain}_runtime-v$version.compact.compressed.wasm
        # echo $url
        wget $url -O data/$chain/V14/$version.wasm
    done
done

wget https://github.com/paritytech/polkadot/releases/download/v0.8.30/kusama_runtime-v2030.compact.wasm -O data/kusama/V12/2030.wasm
wget https://github.com/paritytech/polkadot/releases/download/v0.8.30/polkadot_runtime-v30.compact.wasm -O data/polkadot/V12/2030.wasm

wget https://github.com/paritytech/polkadot/releases/download/v0.8.29/kusama_runtime-v2029.compact.wasm -O data/kusama/V12/2029.wasm
wget https://github.com/paritytech/polkadot/releases/download/v0.8.29/polkadot_runtime-v29.compact.wasm -O data/polkadot/V12/2029.wasm
