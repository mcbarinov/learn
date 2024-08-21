from bip_utils import WifDecoder

priv_key_wif = "cTTQ7HsCMbi9gL1MCG4HqqmgvRAtoSY7sgG9J74b6oCvmSBEEC8T"
priv_key_bytes, pub_key_mode = WifDecoder.Decode(priv_key_wif, net_ver=b"\xef")
print(priv_key_bytes, pub_key_mode)
