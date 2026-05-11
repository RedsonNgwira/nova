#ifndef INCLUDED_SFX2_SOURCE_SIDEBAR_LUMENPANEL_HXX
#define INCLUDED_SFX2_SOURCE_SIDEBAR_LUMENPANEL_HXX

#include <vcl/window.hxx>
#include <vcl/edit.hxx>
#include <vcl/button.hxx>
#include <vcl/vclptr.hxx>

namespace sfx2::sidebar {

class LumenPanel : public vcl::Window
{
public:
    LumenPanel(vcl::Window* pParent);
    virtual ~LumenPanel() override;

    virtual void Paint(vcl::RenderContext& rRenderContext, const tools::Rectangle& rRect) override;
    virtual void Resize() override;

    static char* GetDocContent();

private:
    VclPtr<Edit> m_pInput;
    VclPtr<PushButton> m_pSendBtn;
    OUString m_aResponse;

    DECL_LINK(SendClickHdl, Button*, void);
};

} // end of namespace sfx2::sidebar

#endif
