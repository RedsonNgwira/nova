#include "LumenPanel.hxx"
#include <vcl/settings.hxx>
#include <vcl/rendercontext.hxx>

namespace sfx2::sidebar {

LumenPanel::LumenPanel(vcl::Window* pParent)
    : Window(pParent, WB_TABSTOP)
{
    SetText("Lumen");
    SetBackground(Wallpaper(GetSettings().GetStyleSettings().GetWindowColor()));
}

LumenPanel::~LumenPanel()
{
}

void LumenPanel::Paint(vcl::RenderContext& rRenderContext, const tools::Rectangle& /*rRect*/)
{
    rRenderContext.SetFont(GetSettings().GetStyleSettings().GetLabelFont());
    rRenderContext.SetTextColor(GetSettings().GetStyleSettings().GetLabelTextColor());

    OUString aText("Lumen is ready");
    Size aTextSize(rRenderContext.GetTextWidth(aText), rRenderContext.GetTextHeight());
    Size aWinSize(GetSizePixel());

    Point aPos((aWinSize.Width() - aTextSize.Width()) / 2,
               (aWinSize.Height() - aTextSize.Height()) / 2);

    rRenderContext.DrawText(aPos, aText);
}

void LumenPanel::Resize()
{
    Invalidate();
}

} // end of namespace sfx2::sidebar
