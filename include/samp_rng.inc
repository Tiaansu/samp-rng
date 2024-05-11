#if defined _INC_samp_rng
    #undef _INC_samp_rng
#endif

#if defined _samp_rng_included
    #endinput
#endif

native RandomNumber();
native RandomNumberMax(max);
native RandomNumberMinMax(min, max);