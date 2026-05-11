#include "LumenPanel.hxx"
#include "LumenBridge.h"
#include <vcl/settings.hxx>
#include <vcl/rendercontext.hxx>
#include <vcl/svapp.hxx>

namespace sfx2::sidebar {

LumenPanel::LumenPanel(vcl::Window* pParent)
    : Window(pParent, WB_TABSTOP)
    , m_pInput(VclPtr<Edit>::Create(this, WB_BORDER))
    , m_pSendBtn(VclPtr<PushButton>::Create(this, WB_TABSTOP))
    , m_aResponse("Lumen is ready")
{
    SetText("Lumen");
    SetBackground(Wallpaper(GetSettings().GetStyleSettings().GetWindowColor()));

    m_pInput->Show();
    m_pSendBtn->SetText("Send");
    m_pSendBtn->SetClickHdl(LINK(this, LumenPanel, SendClickHdl));
    m_pSendBtn->Show();

    // Initialize Lumen
    lumen_init("placeholder_key");
    lumen_set_get_content_callback(LumenPanel::GetDocContent);
}

char* LumenPanel::GetDocContent()
{
    // In a real implementation, we would use the UNO API to get the text of the active document.
    // e.g. xModel->getComponentContext()...
    std::string aContent = "This is a placeholder for actual document content retrieved via UNO.";
    char* pRes = static_cast<char*>(malloc(aContent.size() + 1));
    strcpy(pRes, aContent.c_str());
    return pRes;
}

LumenPanel::~LumenPanel()
{
    m_pInput.disposeAndClear();
    m_pSendBtn.disposeAndClear();
}

IMPL_LINK_NOARG(LumenPanel, SendClickHdl, Button*, void)
{
    OUString aQuery = m_pInput->GetText();
    if (aQuery.isEmpty())
        return;

    m_aResponse = "Thinking...";
    Invalidate();
    Application::Yield(); // Force redraw

    OString aQueryUTF8 = OUStringToOString(aQuery, RTL_TEXTENCODING_UTF8);
    char* pRes = lumen_query(aQueryUTF8.getStr());
    
    if (pRes)
    {
        m_aResponse = OUString::createFromAscii(pRes);
        lumen_free_string(pRes);
    }
    else
    {
        m_aResponse = "Error: No response from Lumen";
    }

    m_pInput->SetText("");
    Invalidate();
}

void LumenPanel::Paint(vcl::RenderContext& rRenderContext, const tools::Rectangle& /*rRect*/)
{
    rRenderContext.SetFont(GetSettings().GetStyleSettings().GetLabelFont());
    rRenderContext.SetTextColor(GetSettings().GetStyleSettings().GetLabelTextColor());

    // Draw response text with wrapping (simple for now)
    tools::Rectangle aTextRect(Point(10, 10), Size(GetSizePixel().Width() - 20, 100));
    rRenderContext.DrawText(aTextRect, m_aResponse, DrawTextFlags::MultiLine | DrawTextFlags::WordBreak);
}

void LumenPanel::Resize()
{
    Size aWinSize = GetSizePixel();
    long nMargin = 10;
    long nBtnWidth = 60;
    long nInputHeight = 30;

    m_pInput->SetPosSizePixel(Point(nMargin, aWinSize.Height() - nInputHeight - nMargin),
                              Size(aWinSize.Width() - nBtnWidth - 3 * nMargin, nInputHeight));

    m_pSendBtn->SetPosSizePixel(Point(aWinSize.Width() - nBtnWidth - nMargin, aWinSize.Height() - nInputHeight - nMargin),
                               Size(nBtnWidth, nInputHeight));

    Invalidate();
}

} // end of namespace sfx2::sidebar
