# https://docs.hunter.sh/en/latest/packages/pkg/GTest.html
hunter_add_package(GTest)
find_package(GTest CONFIG REQUIRED)

hunter_add_package(Microsoft.GSL)
find_package(Microsoft.GSL CONFIG REQUIRED)

schnorrkel_add_test(sr25519_test
        sr25519/ds.cpp
        sr25519/derive.cpp
        sr25519/keypair_from_seed.cpp
        sr25519/vrf.cpp
        sr25519/keccak.c
    )
target_link_libraries(sr25519_test
    schnorrkel_crust
    )

schnorrkel_add_test(ed25519_test
        ed25519/test.cpp)
target_link_libraries(ed25519_test
    schnorrkel_crust
    )
