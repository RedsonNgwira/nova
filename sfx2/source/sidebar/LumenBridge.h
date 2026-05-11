#ifndef INCLUDED_SFX2_SOURCE_SIDEBAR_LUMENBRIDGE_H
#define INCLUDED_SFX2_SOURCE_SIDEBAR_LUMENBRIDGE_H

extern "C" {
    typedef char* (*GetDocContentFn)();

    int lumen_init(const char* api_key);
    char* lumen_query(const char* query);
    void lumen_free_string(char* s);
    void lumen_set_get_content_callback(GetDocContentFn cb);
}

#endif
